use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
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

pub trait Dim {
    fn dim(self, dim: bool) -> Block<'static>;
}

impl Dim for Block<'static> {
    fn dim(self, dim: bool) -> Self {
        if dim {
            let style = Style::default().fg(Color::Indexed(8));
            self.border_style(style).style(style)
        } else {
            self
        }
    }
}

pub fn default_block(title: &'static str) -> Block {
    Block::default()
        .border_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .title(title)
}

fn match_char(data: &char) -> bool {
    match *data {
        '\x01'..='\x08' | '\u{10FFFE}'..='\u{10FFFF}' => true,
        _ => false,
    }
}
