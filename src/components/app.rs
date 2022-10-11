use super::notification::{Notification, ToodMsg};
use super::skimmer::Skimmer;
use super::todo_list::TodoList;
use crate::keymap::ToodKeyList;

pub struct App {
    pub todos: TodoList,
    pub keys: ToodKeyList,
    pub mode: InputMode,
    pub skimmer: Skimmer,
    pub notification: Notification,
}

pub enum InputMode {
    Normal,
    Edit,
    Find,
    Move,
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
        self.notification.set(ToodMsg::warn("Removed todo"));
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
        self.todos.transfer_selected_to_input();
        self.enter_mode(InputMode::Edit);
    }

    pub fn enter_mode(&mut self, mode: InputMode) {
        let msg = match mode {
            InputMode::Edit => {
                self.mode = mode;
                "Entered edit mode"
            }
            InputMode::Find => {
                self.mode = mode;
                self.skimmer.skim(None, &self.todos.todos);
                "Entered find mode"
            }
            InputMode::Move => {
                self.mode = mode;
                self.todos.toggle_move_mode();
                "Entered move mode"
            }
            InputMode::Normal => {
                self.reset_state();
                "Entered normal mode"
            }
        };
        self.notification.set(ToodMsg::info(msg));
    }

    pub fn toggle_todo_completed(&mut self) {
        if self.todos.toggle_completed().is_err() {
            self.notification
                .set(ToodMsg::warn("Cannot mark recurring todos as completed"));
            return;
        }
        let toggle_msg = if self.todos.selected().unwrap().finished {
            "Marked todo completed"
        } else {
            "Marked todo not completed"
        };
        self.notification.set(ToodMsg::info(toggle_msg));
    }

    pub fn add_todo(&mut self) {
        self.todos.add_todo();
        let action = if self.todos.new_todo.is_editing_existing {
            "Edited existing todo"
        } else {
            "Added new todo"
        };
        self.reset_state();
        self.notification.set(ToodMsg::info(action));
    }

    pub fn reset_state(&mut self) {
        match self.mode {
            InputMode::Move => self.todos.toggle_move_mode(),
            InputMode::Edit => self.todos.reset_input(),
            InputMode::Find => self.skimmer = Skimmer::default(),
            _ => {}
        }
        self.mode = InputMode::Normal;
    }

    pub fn load_fuzzy_selection(&mut self) {
        if let Some(skim_match) = self.skimmer.selected_match() {
            self.todos.state.select(Some(skim_match.position));
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
