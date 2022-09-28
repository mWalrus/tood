use anyhow::Result;
use serde::{Deserialize, Serialize};
use tui::widgets::ListState;
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;

use crate::TODO_FILE;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Todo {
    #[serde(default = "finished_default")]
    pub finished: bool,
    pub name: String,
    pub description: String,
}

#[inline(always)]
fn finished_default() -> bool {
    false
}

impl Todo {
    pub fn push_text(&mut self, field: &Field, c: char) {
        match field {
            Field::Name => self.name.push(c),
            Field::Description => self.description.push(c),
        }
    }

    pub fn pop(&mut self, field: &Field) {
        match field {
            Field::Name => self.name.pop(),
            Field::Description => self.description.pop(),
        };
    }
}

#[derive(PartialEq, Eq)]
pub enum Field {
    Name,
    Description,
}

impl Default for Field {
    fn default() -> Field {
        Field::Name
    }
}

// FIXME!!!!!!: make use of tui_input crate and its features
pub struct App {
    pub todos: TodoList,
    pub new_todo: Todo,
    pub field: Field,
    pub mode: InputMode,
}

pub enum InputMode {
    Normal,
    Editing,
}

impl Default for App {
    fn default() -> App {
        App {
            todos: TodoList::load(),
            new_todo: Todo::default(),
            field: Field::Name,
            mode: InputMode::Normal,
        }
    }
}

impl App {
    pub fn remove_current_todo(&mut self) {
        self.todos.remove_current();
        self.todos.check_selection();
    }

    pub fn get_current_description(&self) -> &str {
        if let Some(selected) = self.todos.state.selected() {
            &self.todos.todos.get(selected).unwrap().description
        } else {
            ""
        }
    }

    pub fn exit(&self) -> Result<()> {
        // FIXME: saving fails because the array fields
        //        have no name, therefore the deser cant
        //        populate the TodoList with the todos
        let toml = toml::to_string(&self.todos)?;
        std::fs::write(&*TODO_FILE, toml)?;
        Ok(())
    }

    pub fn add_todo(&mut self) {
        self.todos.add_todo(self.new_todo.clone());
        self.field = Field::Name;
        self.mode = InputMode::Normal;
        self.new_todo = Todo::default();
    }
}

#[derive(Deserialize, Serialize)]
pub struct TodoList {
    #[serde(skip, default)]
    pub state: ListState,
    #[serde(default)]
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn load() -> TodoList {
        let todos_string = std::fs::read_to_string(&*TODO_FILE).unwrap();
        let mut todos: TodoList = toml::from_str(&todos_string).unwrap();
        if !todos.todos.is_empty() {
            todos.state.select(Some(0));
        }
        todos
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
            self.state.select(Some(self.state.selected().unwrap() - 1));
        }
    }

    pub fn remove_current(&mut self) {
        if let Some(selected) = self.state.selected() {
            self.todos.remove(selected);
        }
    }

    pub fn add_todo(&mut self, item: Todo) {
        self.todos.push(item);
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
