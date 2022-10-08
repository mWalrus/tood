use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn centered_rect(size: Rect) -> Rect {
    let width = size.width / 2;
    let x = width / 2;
    let height = size.height / 2;
    let y = height / 2;

    Rect {
        x,
        y,
        width,
        height,
    }
}

pub fn default_block(title: &'static str) -> Block {
    Block::default()
        .border_style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .title(title)
}
