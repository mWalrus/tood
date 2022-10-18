use std::thread;

use anyhow::Result;
use kanal::{unbounded, Receiver, Sender};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::EVENT_TIMEOUT;

use super::StaticComponent;

pub struct NotificationComponent {
    pub rx: Receiver<()>,
    tx: Sender<()>,
    pub msg: Option<FlashMsg>,
}

// FIXME: currently the component fails to render whenever there are
//        a lot of actions happening at once.
impl NotificationComponent {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self { rx, tx, msg: None }
    }

    pub fn set(&mut self, msg: FlashMsg) {
        self.msg = Some(msg);
        let tx = self.tx.clone();
        thread::spawn(move || {
            thread::sleep(EVENT_TIMEOUT);
            tx.send(()).unwrap();
        });
    }

    pub fn handle_queue(&mut self) -> Result<()> {
        if (self.rx.try_recv()?).is_some() {
            self.msg = None;
        }
        Ok(())
    }
}

impl StaticComponent for NotificationComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if let Some(msg) = &self.msg {
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
