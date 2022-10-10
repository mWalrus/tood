use chrono::{DateTime, Local};
use serde::Serializer;
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn serialize_local_date<S>(dt: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.to_rfc3339())
}

pub fn serialize_optional_local_date<S>(
    dt: &Option<DateTime<Local>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.unwrap().to_rfc3339())
}

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
