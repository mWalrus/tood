use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::types::{app::App, notification::ToodMsgType};

pub fn draw<B: Backend>(app: &App, f: &mut Frame<B>) {
    if let Some(notif) = &app.notification.msg {
        let notif_span = match notif.level {
            ToodMsgType::Error => {
                Span::styled(&notif.message, Style::default().bg(Color::LightRed))
            }
            ToodMsgType::Warn => Span::styled(
                &notif.message,
                Style::default().bg(Color::Yellow).fg(Color::Black),
            ),
            ToodMsgType::Info => Span::styled(
                &notif.message,
                Style::default().bg(Color::Green).fg(Color::Black),
            ),
        };
        let notif_paragraph =
            Paragraph::new(notif_span).block(Block::default().borders(Borders::NONE));
        let width = notif.message.len() as u16;
        // 2 extra to move it inside the borders
        let size = f.size();
        let x = size.width - width - 2;

        let rect = Rect {
            x,
            y: 1,
            width,
            height: 1,
        };

        f.render_widget(notif_paragraph, rect);
    }
}
