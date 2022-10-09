use crate::types::metadata::TodoMetadata;
use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, ListItem},
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

pub fn metadata_to_list_item(data: TodoMetadata) -> Vec<ListItem<'static>> {
    let mut list_items: Vec<ListItem> = Vec::new();
    for md in data.formatted_vec() {
        let spans = Spans::from(vec![
            Span::styled(md.0, Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(md.1),
        ]);
        list_items.push(ListItem::new(spans));
    }
    list_items
}
