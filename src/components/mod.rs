pub mod due_date;
pub mod notification;
pub mod skimmer;
pub mod todo_input;
pub mod todo_list;
pub mod utils;

use crossterm::event::KeyEvent;
pub use notification::NotificationComponent;
// pub use due_date::DueDateComponent;
pub use skimmer::SkimmerComponent;
pub use todo_input::TodoInputComponent;
pub use todo_list::TodoListComponent;

use anyhow::Result;

use crate::widgets::hint_bar::HintBar;
use tui::{backend::Backend, Frame};

pub trait Component {
    fn draw<B: Backend>(&mut self, _f: &mut Frame<B>) {}
    fn handle_input(&mut self, key: KeyEvent) -> Result<()>;
}

pub trait StaticComponent {
    fn draw<B: Backend>(&mut self, _f: &mut Frame<B>) {}
}

pub trait MainComponent {
    fn draw<B: Backend>(&mut self, _f: &mut Frame<B>, _dim: bool, _hb: HintBar) {}
    fn handle_input(&mut self, key: KeyEvent) -> Result<()>;
}
