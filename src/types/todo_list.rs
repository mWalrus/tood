use serde::{Deserialize, Serialize};
use tui::widgets::ListState;

use super::app::TodoInput;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Todo {
    #[serde(default)]
    pub finished: bool,
    pub name: String,
    pub description: String,
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

    pub fn add_todo(&mut self, item: &TodoInput) {
        let new_todo = Todo {
            finished: false,
            name: item.name.value().into(),
            description: item.description.clone(),
        };
        if item.is_editing_existing {
            if self.has_selection() {
                let sel = self.state.selected().unwrap();
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

    pub fn toggle_completed(&mut self) {
        if self.has_selection() {
            let is_completed = self.todos[self.state.selected().unwrap()].finished;
            self.todos[self.state.selected().unwrap()].finished = !is_completed
        }
    }
}