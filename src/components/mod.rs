use tui::{backend::Backend, layout::Rect, Frame};

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
    fn draw_in_rect<B: Backend>(&self, _f: &mut Frame<B>, _r: &Rect) {}
}
