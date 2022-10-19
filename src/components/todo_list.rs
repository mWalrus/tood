use std::io;

use anyhow::Result;
use chrono::{DateTime, Local};
use crossterm::event::KeyEvent;
use kanal::Sender;
use serde::{Deserialize, Serialize};
use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{List, ListItem, ListState, Paragraph};
use tui::Frame;

use super::notification::FlashMsg;
use super::utils::Dim;
use super::{utils, MainComponent};
use crate::app::{AppMessage, State};
use crate::keys::key_match;
use crate::keys::keymap::SharedKeyList;
use crate::widgets::hint_bar::{BarType, HintBar};

static TIME_FORMAT: &str = "%D %-I:%M %P";

pub enum ListAction {
    Replace(Todo, usize),
    Add(Todo),
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Todo {
    #[serde(default)]
    pub name: String,
    pub description: String,
    pub metadata: TodoMetadata,
}

impl Todo {
    pub fn toggle_finished(&mut self) {
        self.metadata.finished = !self.metadata.finished;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoMetadata {
    pub added_at: DateTime<Local>,
    pub edited_at: Option<DateTime<Local>>,
    pub recurring: bool,
    pub finished: bool,
}

impl TodoMetadata {
    pub fn to_formatted(&self) -> Vec<(&'static str, String)> {
        #[inline(always)]
        fn yes_no(b: bool) -> &'static str {
            if b {
                "yes"
            } else {
                "no"
            }
        }

        let mut c = vec![];
        c.push(("Added: ", self.added_at.format(TIME_FORMAT).to_string()));

        let edited_at = if let Some(ea) = self.edited_at {
            ea.format(TIME_FORMAT).to_string()
        } else {
            "never".into()
        };

        c.push(("Edited: ", edited_at));
        c.push(("Recurring: ", yes_no(self.recurring).into()));
        c.push(("Finished: ", yes_no(self.finished).into()));
        c
    }
}

impl Default for TodoMetadata {
    fn default() -> Self {
        Self {
            added_at: Local::now(),
            edited_at: None,
            recurring: false,
            finished: false,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct TodoListSerde {
    todos: Vec<Todo>,
}

impl From<&TodoListComponent> for TodoListSerde {
    fn from(other: &TodoListComponent) -> Self {
        Self {
            todos: other.todos.clone(),
        }
    }
}

pub struct TodoListComponent {
    pub state: ListState,
    pub todos: Vec<Todo>,
    keys: SharedKeyList,
    hintbars: HintBars,
    move_mode: bool,
    message_tx: Sender<AppMessage>,
}

pub struct HintBars {
    selected: usize,
    items: [HintBar; 5],
}

impl HintBars {
    fn new(keys: SharedKeyList) -> Self {
        Self {
            selected: 0,
            items: [
                HintBar::normal_mode(keys.clone()),
                HintBar::edit_mode(keys.clone()),
                HintBar::find_mode(keys.clone()),
                HintBar::move_mode(keys.clone()),
                HintBar::due_date_mode(keys),
            ],
        }
    }
}

impl TodoListComponent {
    pub fn load(keys: SharedKeyList, message_tx: Sender<AppMessage>) -> Self {
        let todo_data: TodoListSerde = confy::load("tood", Some("todos")).unwrap();
        let mut todo_list = Self {
            state: ListState::default(),
            todos: todo_data.todos,
            keys: keys.clone(),
            hintbars: HintBars::new(keys),
            move_mode: false,
            message_tx,
        };

        todo_list.correct_selection();
        todo_list
    }

    pub fn todos_ref(&self) -> &[Todo] {
        &self.todos
    }

    pub fn add_todo(&mut self, t: Todo) -> Result<()> {
        self.todos.push(t);
        self.state.select(Some(self.todos.len() - 1));
        self.save_to_disk()?;
        Ok(())
    }

    pub fn replace(&mut self, t: Todo, i: usize) -> Result<()> {
        let _ = std::mem::replace(&mut self.todos[i], t);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn save_to_disk(&self) -> io::Result<()> {
        confy::store("tood", Some("todos"), TodoListSerde::from(self)).unwrap();
        Ok(())
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.todos.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.todos.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn correct_selection(&mut self) {
        if self.state.selected().is_none() && !self.todos.is_empty() {
            self.state.select(Some(0));
        } else if self.todos.is_empty() {
            self.state.select(None);
        } else {
            let new_selection = self.state.selected().unwrap().saturating_sub(1);
            self.state.select(Some(new_selection));
        }
    }

    pub fn remove_current(&mut self) -> Result<()> {
        if let Some(selected) = self.state.selected() {
            self.todos.remove(selected);
            self.correct_selection();
            self.save_to_disk().unwrap();
            self.message_tx
                .send(AppMessage::Flash(FlashMsg::info("Removed todo")))?;
            return Ok(());
        }
        self.report_no_selection();
        Ok(())
    }

    pub fn report_no_selection(&self) {
        self.message_tx
            .send(AppMessage::Flash(FlashMsg::err("No todo selected")))
            .unwrap();
    }

    pub fn selected(&self) -> Option<(&Todo, usize)> {
        if let Some(s) = self.state.selected() {
            return Some((&self.todos[s], s));
        }
        None
    }

    pub fn toggle_finished(&mut self) {
        if let Some(s) = self.state.selected() {
            // dont toggle if the todo is recurring
            if self.todos[s].metadata.recurring {
                self.message_tx
                    .send(AppMessage::Flash(FlashMsg::warn(
                        "Can't mark recurring as finished",
                    )))
                    .unwrap();
                return;
            }
            self.todos[s].toggle_finished();
            self.save_to_disk().unwrap();
            let msg = if self.todos[s].metadata.finished {
                "Marked todo as finished"
            } else {
                "Marked todo as unfinished"
            };
            self.message_tx
                .send(AppMessage::Flash(FlashMsg::info(msg)))
                .unwrap();
        } else {
            self.report_no_selection();
        }
    }

    pub fn move_todo_up(&mut self) {
        if let Some(s) = self.state.selected() {
            let new_index = if s == 0 { self.todos.len() - 1 } else { s - 1 };
            self.todos.swap(s, new_index);
            self.state.select(Some(new_index));
            self.save_to_disk().unwrap();
        }
    }
    pub fn move_todo_down(&mut self) {
        if let Some(s) = self.state.selected() {
            let new_index = if s == self.todos.len() - 1 { 0 } else { s + 1 };
            self.todos.swap(s, new_index);
            self.state.select(Some(new_index));
            self.save_to_disk().unwrap();
        }
    }

    #[inline(always)]
    pub fn select(&mut self, selection: usize) {
        self.state.select(Some(selection));
    }

    pub fn load_hintbar(&mut self, bar_type: BarType) {
        self.hintbars.selected = bar_type as usize;
    }
}

impl MainComponent for TodoListComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, dim: bool) {
        let size = f.size();
        let hintbar = &self.hintbars.items[self.hintbars.selected];
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Min(3),
                    Constraint::Length(hintbar.height_required(size.width - 2, size.height)),
                ]
                .as_ref(),
            )
            .split(size);

        let list_items: Vec<ListItem> = self
            .todos
            .iter()
            .map(|t| {
                let (finished, mut fg_style) = if t.metadata.recurring {
                    ("[∞] ", Style::default().fg(Color::Blue))
                } else if t.metadata.finished {
                    ("[x] ", Style::default().fg(Color::Green))
                } else {
                    ("[ ] ", Style::default())
                };

                if dim {
                    fg_style = Style::default();
                }

                let line = finished.to_string() + t.name.as_ref();
                let line = vec![Spans::from(line)];
                ListItem::new(line).style(fg_style)
            })
            .collect();

        let border_style = if self.move_mode {
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().add_modifier(Modifier::BOLD)
        };

        let highlight_style = if dim {
            Style::default()
        } else if self.move_mode {
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .bg(Color::Indexed(8))
                .add_modifier(Modifier::BOLD)
        };

        let items = List::new(list_items)
            .block(
                utils::default_block("Todos")
                    .border_style(border_style)
                    .dim(dim),
            )
            .highlight_style(highlight_style)
            .highlight_symbol("> ");
        f.render_stateful_widget(items, chunks[0], &mut self.state);

        let data_chunks = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Min(30)].as_ref())
            .split(chunks[1]);

        if let Some((t, _)) = self.selected() {
            let description = Paragraph::new(&*t.description)
                .wrap(tui::widgets::Wrap { trim: true })
                .block(utils::default_block("Description").dim(dim));
            f.render_widget(description, data_chunks[0]);

            let mut list_items: Vec<ListItem> = Vec::new();
            for md in t.metadata.to_formatted() {
                let spans = Spans::from(vec![
                    Span::styled(md.0, Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(md.1.to_string()),
                ]);
                list_items.push(ListItem::new(spans));
            }
            let metadata_list =
                List::new(list_items).block(utils::default_block("Metadata").dim(dim));
            f.render_widget(metadata_list, data_chunks[1]);
        } else {
            let placeholder1 =
                Paragraph::new("").block(utils::default_block("Description").dim(dim));
            let placeholder2 = Paragraph::new("").block(utils::default_block("Metadata").dim(dim));
            f.render_widget(placeholder1, data_chunks[0]);
            f.render_widget(placeholder2, data_chunks[1]);
        }
        f.render_widget(hintbar, chunks[2]);
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        if key_match(&key, &self.keys.quit) {
            self.message_tx.send(AppMessage::Quit)?;
        } else if key_match(&key, &self.keys.move_up) {
            if self.move_mode {
                self.move_todo_up();
            } else {
                self.previous();
            }
        } else if key_match(&key, &self.keys.move_down) {
            if self.move_mode {
                self.move_todo_down();
            } else {
                self.next();
            }
        } else if key_match(&key, &self.keys.toggle_completed) {
            self.toggle_finished();
        } else if key_match(&key, &self.keys.add_todo) {
            self.message_tx
                .send(AppMessage::InputState(State::AddTodo))?;
        } else if key_match(&key, &self.keys.edit_todo) {
            self.message_tx
                .send(AppMessage::InputState(State::EditTodo))?;
        } else if key_match(&key, &self.keys.move_mode) {
            self.move_mode = true;
            self.message_tx.send(AppMessage::InputState(State::Move))?;
        } else if key_match(&key, &self.keys.find_mode) {
            self.message_tx.send(AppMessage::InputState(State::Find))?;
        } else if key_match(&key, &self.keys.remove_todo) {
            self.remove_current()?;
        } else if key_match(&key, &self.keys.submit) && self.move_mode {
            self.move_mode = false;
        }
        Ok(())
    }
}
