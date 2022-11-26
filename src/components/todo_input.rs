use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use crossterm::event::{Event, KeyEvent};
use kanal::Sender;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;

use crate::{
    app::{AppMessage, State},
    keys::{key_match, keymap::SharedKeyList},
};

use super::{
    todo_list::{ListAction, Todo, TodoMetadata},
    utils, Component,
};

#[derive(Clone)]
pub struct TodoInputComponent {
    pub name: Input,
    pub description: String,
    pub finished: bool,
    pub metadata: TodoMetadata,
    pub is_editing_existing: bool,
    todo_index: usize,
    keys: SharedKeyList,
    message_tx: Sender<AppMessage>,
}

impl From<TodoInputComponent> for Todo {
    fn from(other: TodoInputComponent) -> Self {
        let edited_at = if !other.is_editing_existing {
            None
        } else {
            Some(Local::now())
        };

        Self {
            name: other.name.value().to_string(),
            description: other.description,
            metadata: TodoMetadata {
                edited_at,
                ..other.metadata
            },
        }
    }
}

impl TodoInputComponent {
    pub fn new(keys: SharedKeyList, message_tx: Sender<AppMessage>) -> Self {
        Self {
            name: Input::default(),
            description: String::default(),
            finished: false,
            metadata: TodoMetadata::default(),
            is_editing_existing: false,
            todo_index: 0,
            keys,
            message_tx,
        }
    }

    pub fn populate_with(&mut self, todo: &Todo, i: usize) {
        self.name = Input::from(todo.name.clone());
        self.description = todo.description.to_string();
        self.metadata = todo.metadata.clone();
        self.is_editing_existing = true;
        self.todo_index = i;
    }

    pub fn set_due_date(&mut self, dt: NaiveDateTime) {
        self.metadata.due_date = Some(dt);
    }

    pub fn get_due_date(&self) -> Option<NaiveDateTime> {
        self.metadata.due_date
    }

    pub fn clear(&mut self) {
        self.name = Input::default();
        self.description.clear();
        self.metadata = TodoMetadata::default();
        self.is_editing_existing = false;
    }
}

impl Component for TodoInputComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, _dim: bool) {
        let rect = utils::centered_rect(f.size());

        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Max(10)].as_ref())
            .split(rect);

        let name_input = &self.name;
        let width = chunks[0].width.max(3) - 3;
        let scroll = (name_input.cursor() as u16).max(width) - width;
        let name_input = Paragraph::new(name_input.value())
            .scroll((0, scroll))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
                    .title("Name"),
            );

        let width = chunks[1].width.max(3) - 3;
        let desc_input = Paragraph::new(&*self.description)
            .wrap(tui::widgets::Wrap { trim: true })
            .block(utils::default_block("Description"));

        f.render_widget(Clear, chunks[0]);
        f.render_widget(Clear, chunks[1]);

        f.render_widget(name_input, chunks[0]);
        f.render_widget(desc_input, chunks[1]);
        f.set_cursor(
            chunks[0].x + (self.name.cursor() as u16).min(width) + 1,
            chunks[0].y + 1,
        );
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        if key_match(&key, &self.keys.back) {
            // abort current edit
            self.clear();
            self.message_tx
                .send(AppMessage::InputState(State::Normal))?;
        } else if key_match(&key, &self.keys.submit) {
            if self.is_editing_existing {
                self.message_tx
                    .send(AppMessage::UpdateList(ListAction::Replace(
                        self.clone().into(),
                        self.todo_index,
                    )))?;
            } else {
                self.message_tx
                    .send(AppMessage::UpdateList(ListAction::Add(self.clone().into())))?;
            }
        } else if key_match(&key, &self.keys.external_editor) {
            let desc = edit::edit(&self.description)?;
            self.description = desc;
            self.message_tx.send(AppMessage::RestoreTerminal)?;
        } else if key_match(&key, &self.keys.mark_recurring) {
            self.metadata.recurring = !self.metadata.recurring;
        } else if key_match(&key, &self.keys.open_calendar) {
            self.message_tx
                .send(AppMessage::InputState(State::DueDate))?;
        } else {
            input_backend::to_input_request(Event::Key(key)).and_then(|r| self.name.handle(r));
        }
        Ok(())
    }
}
