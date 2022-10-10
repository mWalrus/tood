use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::Component;

pub struct Notification {
    pub rx: Receiver<u8>,
    tx: Sender<u8>,
    pub msg: Option<ToodMsg>,
}

impl Notification {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { rx, tx, msg: None }
    }

    pub fn set(&mut self, msg: ToodMsg) {
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

impl Component for Notification {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if let Some(msg) = &self.msg {
            let notif_span = match msg.level {
                ToodMsgType::Error => {
                    Span::styled(&msg.message, Style::default().bg(Color::LightRed))
                }
                ToodMsgType::Warn => Span::styled(
                    &msg.message,
                    Style::default().bg(Color::Yellow).fg(Color::Black),
                ),
                ToodMsgType::Info => Span::styled(
                    &msg.message,
                    Style::default().bg(Color::Green).fg(Color::Black),
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
pub struct ToodMsg {
    pub message: String,
    pub level: ToodMsgType,
}

#[derive(Clone)]
pub enum ToodMsgType {
    Error,
    Warn,
    Info,
}

impl ToodMsg {
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
