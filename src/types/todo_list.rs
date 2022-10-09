use chrono::Local;
use crossterm::event::{Event, KeyEvent};
use serde::{Deserialize, Serialize};
use tui::widgets::ListState;
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;

use super::metadata::TodoMetadata;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Todo {
    #[serde(default)]
    pub finished: bool,
    pub name: String,
    pub description: String,
    pub metadata: TodoMetadata,
}

#[derive(Default, Debug, Clone)]
pub struct TodoInput {
    pub name: Input,
    pub description: String,
    pub recurring: bool,
    pub is_editing_existing: bool,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TodoList {
    #[serde(skip, default)]
    pub state: ListState,
    #[serde(default, rename(serialize = "todos", deserialize = "todos"))]
    pub todos: Vec<Todo>,
    #[serde(skip, default)]
    pub new_todo: TodoInput,
}

impl TodoList {
    pub fn load() -> TodoList {
        let mut todo_list: TodoList = confy::load("tood", Some("todos")).unwrap();
        todo_list.correct_selection();
        todo_list
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
            let new_selection = self.state.selected().unwrap().checked_sub(1).unwrap_or(0);
            self.state.select(Some(new_selection));
        }
    }

    pub fn remove_current(&mut self) {
        if let Some(selected) = self.state.selected() {
            self.todos.remove(selected);
            self.correct_selection();
        }
    }

    pub fn add_todo(&mut self) {
        let mut new_todo = Todo {
            finished: false,
            name: self.new_todo.name.value().into(),
            description: self.new_todo.description.clone(),
            metadata: TodoMetadata::default(),
        };
        if self.new_todo.is_editing_existing {
            if self.has_selection() {
                let sel = self.state.selected().unwrap();
                let original_metadata = self.todos[sel].metadata.clone();
                new_todo.metadata = TodoMetadata {
                    edited_at: Some(Local::now()),
                    recurring: self.new_todo.recurring,
                    ..original_metadata
                };
                let _ = std::mem::replace(&mut self.todos[sel], new_todo);
                return;
            } else {
                // NOTE: should be impossible to get here
                unreachable!()
            }
        }
        self.todos.push(new_todo);
        if self.state.selected().is_none() {
            self.state.select(Some(0))
        }
    }

    pub fn selected(&self) -> Option<&Todo> {
        if self.has_selection() {
            return Some(&self.todos[self.state.selected().unwrap()]);
        }
        None
    }

    pub fn has_selection(&self) -> bool {
        self.state.selected().is_some()
    }

    pub fn toggle_completed(&mut self) -> Result<(), ()> {
        if self.has_selection() {
            let selected_todo = &self.todos[self.state.selected().unwrap()];
            // dont toggle if the todo is recurring
            if selected_todo.metadata.recurring {
                return Err(());
            }
            let finished = selected_todo.finished;
            self.todos[self.state.selected().unwrap()].finished = !finished;
        }
        Ok(())
    }

    pub fn handle_input(&mut self, ev: KeyEvent) {
        input_backend::to_input_request(Event::Key(ev)).and_then(|r| self.new_todo.name.handle(r));
    }

    pub fn reset_input(&mut self) {
        self.new_todo = TodoInput::default();
    }

    pub fn populate_new_todo(&mut self) {
        if !self.has_selection() {
            return;
        }
        let current_todo = &self.todos[self.state.selected().unwrap()];
        let new_todo = TodoInput {
            name: Input::new(current_todo.name.to_string()),
            description: current_todo.description.to_string(),
            is_editing_existing: true,
            recurring: current_todo.metadata.recurring,
        };
        self.new_todo = new_todo;
    }

    pub fn toggle_recurring(&mut self) -> bool {
        self.new_todo.recurring = !self.new_todo.recurring;
        self.new_todo.recurring
    }
}
