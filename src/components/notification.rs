use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::StaticComponent;

pub enum NotificationType {
    Info,
}

pub struct NotificationComponent {
    pub rx: Receiver<u8>,
    tx: Sender<u8>,
    pub msg: Option<NotificationMessage>,
}

impl NotificationComponent {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { rx, tx, msg: None }
    }

    pub fn set(&mut self, msg: NotificationMessage) {
        self.msg = Some(msg);
        let tx = self.tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            tx.send(0).unwrap();
        });
    }

    pub fn clear(&mut self) {
        self.msg = None;
    }
}

impl StaticComponent for NotificationComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if let Some(msg) = &self.msg {
            let notif_span = match msg.level {
                ToodMsgType::Error => Span::styled(
                    &msg.message,
                    Style::default()
                        .bg(Color::LightRed)
                        .add_modifier(Modifier::BOLD),
                ),
                ToodMsgType::Warn => Span::styled(
                    &msg.message,
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                ),
                ToodMsgType::Info => Span::styled(
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
pub struct NotificationMessage {
    pub message: String,
    pub level: ToodMsgType,
}

#[derive(Clone)]
pub enum ToodMsgType {
    Error,
    Warn,
    Info,
}

impl NotificationMessage {
    pub fn warn<T: ToString>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
            level: ToodMsgType::Warn,
        }
    }
    pub fn err<T: ToString>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
            level: ToodMsgType::Error,
        }
    }
    pub fn info<T: ToString>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
            level: ToodMsgType::Info,
        }
    }
}
