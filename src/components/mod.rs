use tui::{backend::Backend, Frame};

use hint_bar::HintBar;

pub mod app;
pub mod hint_bar;
pub mod metadata;
pub mod notification;
pub mod skimmer;
pub mod todo_input;
pub mod todo_list;
mod utils;

pub trait Component {
    fn draw<B: Backend>(&mut self, _f: &mut Frame<B>) {}
}

pub trait MainComponent {
    fn draw<B: Backend>(&mut self, _f: &mut Frame<B>, _dim: bool, _hb: HintBar) {}
}
