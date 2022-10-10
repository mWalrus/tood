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

    pub fn save_to_disk(&self) -> io::Result<()> {
        confy::store("tood", Some("todos"), &self.todos).unwrap();
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
        if self.todos.toggle_completed().is_err() {
            self.notification
                .set(ToodMsg::warn("Cannot mark recurring todos as completed"));
            return;
        }
        self.save_to_disk().unwrap();
        let toggle_msg = if self.todos.selected().unwrap().finished {
            "Marked todo completed"
        } else {
            "Marked todo not completed"
        };
        self.notification.set(ToodMsg::info(toggle_msg));
    }

    pub fn add_todo(&mut self) {
        self.todos.add_todo();
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

    pub fn toggle_recurring(&mut self) {
        let new_state = self.todos.toggle_recurring();
        let msg = if new_state {
            "Marked todo recurring"
        } else {
            "Marked todo nonrecurring"
        };

        self.notification.set(ToodMsg::info(msg));
    }
}
