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

use tui::{backend::Backend, Frame};

pub static HIGHLIGHT_SYMBOL: &str = " > ";

pub trait Component {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, dim: bool);
    fn handle_input(&mut self, _key: KeyEvent) -> Result<()> {
        Ok(())
    }
}
