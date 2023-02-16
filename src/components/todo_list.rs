use std::cell::Cell;
use std::error::Error;
use std::io;

use anyhow::Result;
use chrono::{DateTime, Local, NaiveDateTime};
use crossterm::event::KeyEvent;
use kanal::Sender;
use serde::{Deserialize, Serialize};
use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{List, ListItem, Paragraph};
use tui::Frame;
use tui_utils::blocks::Dim;
use tui_utils::component::Component;
use tui_utils::keys::key_match;
use tui_utils::state::{Boundary, BoundedState, StateWrap};
use tui_utils::style::highlight_style;
use tui_utils::LIST_HIGHLIGHT_SYMBOL;

use super::notification::FlashMsg;
use super::utils;
use crate::app::{AppMessage, AppState};
use crate::keys::keymap::SharedKeyList;
use crate::widgets::hint_bar::{BarType, HintBar};
use crate::widgets::stateful_paragraph::{ParagraphState, ScrollPos, StatefulParagraph};

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
    pub due_date: Option<NaiveDateTime>,
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

        let mut c = Vec::with_capacity(5);
        c.push(("Added: ", self.added_at.format(TIME_FORMAT).to_string()));

        let edited_at = if let Some(ea) = self.edited_at {
            ea.format(TIME_FORMAT).to_string()
        } else {
            "never".into()
        };

        let due_date = if let Some(dd) = self.due_date {
            dd.format(TIME_FORMAT).to_string()
        } else {
            "not set".into()
        };

        c.push(("Edited: ", edited_at));
        c.push(("Due date: ", due_date));
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
            due_date: None,
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
    pub list_state: BoundedState,
    paragraph_state: Cell<ParagraphState>,
    pub todos: Vec<Todo>,
    keys: SharedKeyList,
    hintbars: HintBars,
    move_mode: bool,
    flash_tx: Sender<FlashMsg>,
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
                HintBar::move_mode(keys.clone()),
                HintBar::find_mode(keys.clone()),
                HintBar::due_date_mode(keys),
            ],
        }
    }
}

impl TodoListComponent {
    pub fn load(keys: SharedKeyList, flash_tx: Sender<FlashMsg>) -> Self {
        let todo_data: TodoListSerde = confy::load("tood", Some("todos")).unwrap();

        let b = Boundary::from(&todo_data.todos);
        let mut state = BoundedState::new(b, StateWrap::Enable);

        // only set a selection if the boundary is not empty
        if !b.is_empty() {
            state.first();
        }

        Self {
            list_state: state,
            paragraph_state: Cell::new(ParagraphState::default()),
            todos: todo_data.todos,
            keys: keys.clone(),
            hintbars: HintBars::new(keys),
            move_mode: false,
            flash_tx,
        }
    }

    pub fn todos_ref(&self) -> &[Todo] {
        &self.todos
    }

    pub fn add_todo(&mut self, t: Todo) -> Result<()> {
        self.todos.push(t);
        self.list_state
            .update_upper_and_select(self.todos.len() - 1);
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
        self.list_state.next()
    }

    pub fn previous(&mut self) {
        self.list_state.prev()
    }

    pub fn remove_current(&mut self) -> Result<()> {
        if let Some(selected) = self.list_state.inner().selected() {
            self.todos.remove(selected);
            self.list_state.update_boundary_from_vec(&self.todos);
            self.save_to_disk().unwrap();
            self.flash_tx.send(FlashMsg::info("Removed todo"))?;
            return Ok(());
        }
        self.report_no_selection();
        Ok(())
    }

    pub fn report_no_selection(&self) {
        self.flash_tx
            .send(FlashMsg::err("No todo selected"))
            .unwrap();
    }

    pub fn selected(&self) -> Option<(&Todo, usize)> {
        if let Some(s) = self.list_state.inner().selected() {
            return Some((&self.todos[s], s));
        }
        None
    }

    pub fn set_description_scroll(&self, pos: u16) -> bool {
        let mut state = self.paragraph_state.get();

        let new_scroll_pos = pos.min(
            state
                .lines()
                .saturating_sub(state.height().saturating_sub(2)),
        );

        state.set_scroll(ScrollPos {
            x: 0,
            y: new_scroll_pos,
        });
        self.paragraph_state.set(state);
        true
    }

    pub fn toggle_finished(&mut self) {
        if let Some(s) = self.list_state.inner().selected() {
            // dont toggle if the todo is recurring
            if self.todos[s].metadata.recurring {
                self.flash_tx
                    .send(FlashMsg::warn("Can't mark recurring as finished"))
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
            self.flash_tx.send(FlashMsg::info(msg)).unwrap();
        } else {
            self.report_no_selection();
        }
    }

    pub fn move_todo_up(&mut self) {
        if let Some(s) = self.list_state.inner().selected() {
            let new_index = if s == 0 { self.todos.len() - 1 } else { s - 1 };
            self.todos.swap(s, new_index);
            self.list_state.select(new_index).unwrap();
            self.save_to_disk().unwrap();
        }
    }
    pub fn move_todo_down(&mut self) {
        if let Some(s) = self.list_state.inner().selected() {
            let new_index = if s == self.todos.len() - 1 { 0 } else { s + 1 };
            self.todos.swap(s, new_index);
            self.list_state.select(new_index).unwrap();
            self.save_to_disk().unwrap();
        }
    }

    #[inline(always)]
    pub fn select(&mut self, selection: usize) {
        self.list_state.select(selection).unwrap();
    }

    pub fn load_hintbar(&mut self, bar_type: BarType) {
        self.hintbars.selected = bar_type as usize;
    }
}

impl Component for TodoListComponent {
    type Message = AppMessage;
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
            highlight_style()
        };

        let items = List::new(list_items)
            .block(
                utils::default_block("Todos")
                    .border_style(border_style)
                    .dim(dim),
            )
            .highlight_style(highlight_style)
            .highlight_symbol(LIST_HIGHLIGHT_SYMBOL);
        f.render_stateful_widget(items, chunks[0], self.list_state.inner_mut());

        let data_chunks = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Min(30)].as_ref())
            .split(chunks[1]);

        if let Some((t, _)) = self.selected() {
            let description = StatefulParagraph::new(&*t.description)
                .style(Style::default())
                .block(utils::default_block("Description").dim(dim));

            let mut p_state = self.paragraph_state.get();

            f.render_stateful_widget(description, data_chunks[0], &mut p_state);

            self.paragraph_state.set(p_state);

            self.set_description_scroll(p_state.scroll().y);

            let formatted_metadata = t.metadata.to_formatted();
            let mut list_items: Vec<ListItem> = Vec::with_capacity(formatted_metadata.len());
            for md in formatted_metadata {
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
                Paragraph::new("bruh").block(utils::default_block("Description").dim(dim));
            let placeholder2 = Paragraph::new("").block(utils::default_block("Metadata").dim(dim));
            f.render_widget(placeholder1, data_chunks[0]);
            f.render_widget(placeholder2, data_chunks[1]);
        }
        f.render_widget(hintbar, chunks[2]);
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<Self::Message, Box<dyn Error>> {
        if key_match(&key, &self.keys.quit) {
            return Ok(AppMessage::Quit);
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
            return Ok(AppMessage::InputState(AppState::AddTodo));
        } else if key_match(&key, &self.keys.edit_todo) {
            return Ok(AppMessage::InputState(AppState::EditTodo));
        } else if key_match(&key, &self.keys.move_mode) {
            self.move_mode = true;
            return Ok(AppMessage::InputState(AppState::Move));
        } else if key_match(&key, &self.keys.find_mode) {
            return Ok(AppMessage::InputState(AppState::Find));
        } else if key_match(&key, &self.keys.remove_todo) {
            self.remove_current()?;
        } else if key_match(&key, &self.keys.submit) && self.move_mode {
            self.move_mode = false;
            return Ok(AppMessage::InputState(AppState::Normal));
        }
        Ok(AppMessage::NoAction)
    }
}
