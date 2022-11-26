use std::sync::{Arc, Mutex};
use std::thread;

use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::EVENT_TIMEOUT;

use super::Component;

pub struct NotificationComponent {
    pub msg: Arc<Mutex<Option<FlashMsg>>>,
}

impl NotificationComponent {
    pub fn new() -> Self {
        Self {
            msg: Arc::new(Mutex::new(None)),
        }
    }

    pub fn flash(&mut self, msg: FlashMsg) {
        let msg_clone = self.msg.clone();
        thread::spawn(move || {
            if let Ok(mut m) = msg_clone.lock() {
                *m = Some(msg);
            }
            thread::sleep(EVENT_TIMEOUT);
            if let Ok(mut m) = msg_clone.lock() {
                *m = None;
            }
        });
    }
}

impl Component for NotificationComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, _dim: bool) {
        if let Ok(m) = self.msg.clone().try_lock() {
            if m.is_none() {
                return;
            }
            let msg = m.as_ref().unwrap();
            let notif_span = match msg.level {
                MsgType::Error => Span::styled(
                    &msg.message,
                    Style::default()
                        .bg(Color::LightRed)
                        .add_modifier(Modifier::BOLD),
                ),
                MsgType::Warn => Span::styled(
                    &msg.message,
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                ),
                MsgType::Info => Span::styled(
                    &msg.message,
                    Style::default()
                        .bg(Color::Green)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                ),
            };
            let notif_paragraph =
                Paragraph::new(notif_span).block(Block::default().borders(Borders::NONE));
            let width = msg.message.len() as u16;
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
}

#[derive(Clone)]
pub struct FlashMsg {
    pub message: String,
    pub level: MsgType,
}

#[derive(Clone)]
pub enum MsgType {
    Error,
    Warn,
    Info,
}

impl FlashMsg {
    pub fn warn<T: ToString>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
            level: MsgType::Warn,
        }
    }
    pub fn err<T: ToString>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
            level: MsgType::Error,
        }
    }
    pub fn info<T: ToString>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
            level: MsgType::Info,
        }
    }
}
