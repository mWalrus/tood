pub mod app;
pub mod metadata;
pub mod notification;
pub mod skimmer;
pub mod todo_input;
pub mod todo_list;
pub mod utils;

use crate::widgets::hint_bar::HintBar;
use tui::{backend::Backend, Frame};

pub trait Component {
    fn draw<B: Backend>(&mut self, _f: &mut Frame<B>) {}
}

pub trait MainComponent {
    fn draw<B: Backend>(&mut self, _f: &mut Frame<B>, _dim: bool, _hb: HintBar) {}
}
