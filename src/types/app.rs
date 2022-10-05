use super::notification::{Notification, ToodMsg};
use super::todo_list::TodoList;
use crate::keymap::ToodKeyList;

use crossterm::event::{Event, KeyEvent};
use std::io;
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;

pub struct App {
    pub todos: TodoList,
    pub new_todo: TodoInput,
    pub keys: ToodKeyList,
    pub mode: InputMode,
    pub notification: Notification,
}

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Default)]
pub struct TodoInput {
    pub name: Input,
    pub description: String,
    pub is_editing_existing: bool,
}

impl Default for App {
    fn default() -> App {
        App {
            todos: TodoList::load(),
            new_todo: TodoInput::default(),
            keys: ToodKeyList::default(),
            mode: InputMode::Normal,
            notification: Notification::new(),
        }
    }
}

impl App {
    pub fn remove_current_todo(&mut self) {
        self.todos.remove_current();
        self.save_to_disk().unwrap();
        self.notification.set(ToodMsg::warn("Removed todo"));
    }

    pub fn get_current_description(&self) -> &str {
        if let Some(selected) = self.todos.state.selected() {
            &self.todos.todos[selected].description
        } else {
            ""
        }
    }

    pub fn save_to_disk(&self) -> io::Result<()> {
        confy::store("tood", Some("todos.toml"), &self.todos).unwrap();
        Ok(())
    }

    pub fn handle_input_event(&mut self, e: KeyEvent) {
        input_backend::to_input_request(Event::Key(e)).and_then(|r| self.new_todo.name.handle(r));
    }

    pub fn edit_description(&mut self) {
        let desc = edit::edit(&self.new_todo.description).unwrap();
        self.new_todo.description = desc;
    }

    pub fn edit_todo(&mut self) {
        if let Some(current_todo) = self.todos.selected() {
            self.new_todo.name = Input::new(current_todo.name.to_string());
            self.new_todo.description = current_todo.description.to_string();
            self.new_todo.is_editing_existing = true;
            self.mode = InputMode::Editing;
            return;
        }
        self.notification.set(ToodMsg::err("No todo selected"));
    }

    pub fn toggle_todo_completed(&mut self) {
        self.todos.toggle_completed();
        self.save_to_disk().unwrap();
        let toggle_msg = if self.todos.selected().unwrap().finished {
            "Marked todo completed"
        } else {
            "Marked todo not completed"
        };
        self.notification.set(ToodMsg::info(toggle_msg));
    }

    pub fn add_todo(&mut self) {
        self.todos.add_todo(&self.new_todo);
        self.save_to_disk().unwrap();
        let action = if self.new_todo.is_editing_existing {
            "Edited existing todo"
        } else {
            "Added new todo"
        };
        self.reset_state();
        self.notification.set(ToodMsg::info(action));
    }

    pub fn reset_state(&mut self) {
        self.mode = InputMode::Normal;
        self.new_todo = TodoInput::default();
    }
}
