use super::notification::{Notification, ToodMsg};
use super::skimmer::Skimmer;
use super::todo_list::TodoList;
use crate::keymap::ToodKeyList;

use std::io;

pub struct App {
    pub todos: TodoList,
    pub keys: ToodKeyList,
    pub mode: InputMode,
    pub skimmer: Skimmer,
    pub notification: Notification,
}

pub enum InputMode {
    Normal,
    Editing,
    Find,
}

impl Default for App {
    fn default() -> App {
        App {
            todos: TodoList::load(),
            keys: ToodKeyList::default(),
            mode: InputMode::Normal,
            skimmer: Skimmer::default(),
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

    pub fn edit_description(&mut self) {
        let desc = edit::edit(&self.todos.new_todo.description).unwrap();
        self.todos.new_todo.description = desc;
    }

    pub fn edit_todo(&mut self) {
        if self.todos.selected().is_none() {
            self.notification.set(ToodMsg::err("No todo selected"));
            return;
        }
        self.todos.populate_new_todo();
        self.mode = InputMode::Editing;
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
        self.todos.add_todo(self.todos.new_todo.clone());
        self.save_to_disk().unwrap();
        let action = if self.todos.new_todo.is_editing_existing {
            "Edited existing todo"
        } else {
            "Added new todo"
        };
        self.reset_state();
        self.notification.set(ToodMsg::info(action));
    }

    pub fn reset_state(&mut self) {
        self.mode = InputMode::Normal;
        self.todos.reset_input();
        self.skimmer = Skimmer::default();
    }

    pub fn load_fuzzy_selection(&mut self) {
        if let Some(selection) = self.skimmer.state.selected() {
            let skimmer_input = self.skimmer.input.value();

            let mut found_todo_indices: Vec<usize> = Vec::new();
            self.todos.todos.iter().enumerate().for_each(|(i, t)| {
                if t.name.contains(&skimmer_input) {
                    found_todo_indices.push(i);
                }
            });

            let selected_todo = found_todo_indices[selection];
            self.todos.state.select(Some(selected_todo));
        }
        self.skimmer = Skimmer::default();
    }
}
