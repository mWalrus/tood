use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
};
use tui_utils::rect::centered_rect;

pub fn calendar_rect(size: Rect) -> Rect {
    let max_width = 30; // 4 * 7 + 2 (7 columns with 4 width + 2 size to account for borders)
    let max_height = 18; // 2 * 7 (7 rows with 2 height + 2 size to account for borders)
    if size.width < max_width || size.height < max_height {
        centered_rect(size)
    } else {
        Rect {
            x: (size.width / 2).saturating_sub(max_width / 2),
            y: (size.height / 2).saturating_sub(max_height / 2),
            width: max_width,
            height: max_height,
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
