use anyhow::Result;
use crossterm::event::{Event, KeyEvent};
use serde::{Deserialize, Serialize};
use toml::Value;
use tui::widgets::ListState;
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;

use crate::TODO_FILE;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Todo {
    #[serde(default)]
    pub finished: bool,
    pub name: String,
    pub description: String,
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

pub struct App {
    pub todos: TodoList,
    pub new_todo: TodoInput,
    pub field: Field,
    pub mode: InputMode,
}

#[derive(Default)]
pub struct TodoInput {
    pub name: Input,
    pub description: Input,
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

    pub fn save_to_disk(&self) -> Result<()> {
        // FIXME: saving fails because the array fields
        //        have no name, therefore the deser cant
        //        populate the TodoList with the todos
        let toml = toml::to_string(&self.todos)?;
        std::fs::write(&*TODO_FILE, toml)?;
        Ok(())
    }

    pub fn handle_input_event(&mut self, e: KeyEvent) {
        input_backend::to_input_request(Event::Key(e)).and_then(|r| match self.field {
            Field::Name => self.new_todo.name.handle(r),
            Field::Description => self.new_todo.description.handle(r),
        });
    }

    pub fn add_todo(&mut self) {
        self.todos.add_todo(&self.new_todo);
        self.field = Field::Name;
        self.mode = InputMode::Normal;
        // reset input fields
        self.new_todo.name.reset();
        self.new_todo.description.reset();
        self.save_to_disk().unwrap();
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TodoList {
    #[serde(skip, default)]
    pub state: ListState,
    #[serde(default, rename(serialize = "todos", deserialize = "todos"))]
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn load() -> TodoList {
        // FIXME: what the fuck!!!!
        let todos_string = std::fs::read_to_string(&*TODO_FILE).unwrap();
        println!("{todos_string}");
        let mut todos: Value = toml::from_str(&todos_string).unwrap();
        println!("{todos:#?}");
        std::process::exit(0);
        // if !todos.todos.is_empty() {
        //     todos.state.select(Some(0));
        // }
        // todos
        TodoList {
            state: ListState::default(),
            todos: Vec::new(),
        }
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

    pub fn add_todo(&mut self, item: &TodoInput) {
        let new_todo = Todo {
            finished: false,
            name: item.name.value().into(),
            description: item.description.value().into(),
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
