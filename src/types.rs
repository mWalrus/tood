use anyhow::Result;
use crossterm::event::{Event, KeyEvent};
use serde::{Deserialize, Serialize};
use tui::widgets::ListState;
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;

use crate::keymap::ToodKeyList;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Todo {
    #[serde(default)]
    pub finished: bool,
    pub name: String,
    pub description: String,
}

pub struct App {
    pub todos: TodoList,
    pub new_todo: TodoInput,
    pub keys: ToodKeyList,
    pub mode: InputMode,
}

#[derive(Default)]
pub struct TodoInput {
    pub name: Input,
    pub description: String,
}

pub enum InputMode {
    Normal,
    Editing,
}

impl Default for App {
    fn default() -> App {
        App {
            todos: TodoList::load(),
            new_todo: TodoInput::default(),
            keys: ToodKeyList::default(),
            mode: InputMode::Normal,
        }
    }
}

impl App {
    pub fn remove_current_todo(&mut self) {
        self.todos.remove_current();
        self.todos.check_selection();
        self.save_to_disk().unwrap();
    }

    pub fn get_current_description(&self) -> &str {
        if let Some(selected) = self.todos.state.selected() {
            &self.todos.todos.get(selected).unwrap().description
        } else {
            ""
        }
    }

    pub fn save_to_disk(&self) -> Result<()> {
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

    pub fn toggle_todo_completed(&mut self) {
        self.todos.toggle_completed();
        self.save_to_disk().unwrap();
    }

    pub fn add_todo(&mut self) {
        self.todos.add_todo(&self.new_todo);
        self.reset_state();
        self.save_to_disk().unwrap();
    }

    pub fn reset_state(&mut self) {
        self.mode = InputMode::Normal;
        self.new_todo.name.reset();
        self.new_todo.description = String::new();
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TodoList {
    #[serde(skip, default)]
    pub state: ListState,
    #[serde(default, rename(serialize = "todos", deserialize = "todos"))]
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn load() -> TodoList {
        let mut todo_list: TodoList = confy::load("tood", Some("todos.toml")).unwrap();
        if !todo_list.todos.is_empty() {
            todo_list.state.select(Some(0));
        }
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

    pub fn check_selection(&mut self) {
        if self.todos.is_empty() {
            self.state.select(None);
        } else if self.todos.len() == 1 {
            self.state.select(Some(0));
        } else {
            let new_selection = self.state.selected().unwrap().checked_sub(1).unwrap_or(0);
            self.state.select(Some(new_selection));
        }
    }

    pub fn remove_current(&mut self) {
        if let Some(selected) = self.state.selected() {
            self.todos.remove(selected);
        }
    }

    pub fn add_todo(&mut self, item: &TodoInput) {
        let new_todo = Todo {
            finished: false,
            name: item.name.value().into(),
            description: item.description.clone(),
        };
        self.todos.push(new_todo);
        if self.state.selected().is_none() {
            self.state.select(Some(0))
        }
    }

    pub fn has_selection(&self) -> bool {
        self.state.selected().is_some()
    }

    pub fn toggle_completed(&mut self) {
        if self.has_selection() {
            let is_completed = self.todos[self.state.selected().unwrap()].finished;
            self.todos[self.state.selected().unwrap()].finished = !is_completed
        }
    }
}
