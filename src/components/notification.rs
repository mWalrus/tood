use crate::theme::theme::SharedTheme;
use crate::EVENT_TIMEOUT;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::sync::{Arc, Mutex};
use std::thread;
use tui_utils::component::Component;

pub struct NotificationComponent {
    pub msg: Arc<Mutex<Option<FlashMsg>>>,
    theme: SharedTheme,
}

impl NotificationComponent {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            msg: Arc::new(Mutex::new(None)),
            theme,
        }
    }

    pub fn flash(&mut self, msg: FlashMsg) {
        let msg_clone = self.msg.clone();
        thread::spawn(move || {
            let level = msg.level.clone();

            if let Ok(mut m) = msg_clone.lock() {
                *m = Some(msg);
            }

            let timeout = if level == MsgType::Error {
                EVENT_TIMEOUT * 2
            } else {
                EVENT_TIMEOUT
            };

            thread::sleep(timeout);

            if let Ok(mut m) = msg_clone.lock() {
                *m = None;
            }
        });
    }
}

impl Component for NotificationComponent {
    // no need for handling input or sending messages
    type Message = ();

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
                        .bg(self.theme.flash_err_bg)
                        .fg(self.theme.flash_err_fg)
                        .add_modifier(Modifier::BOLD),
                ),
                MsgType::Warn => Span::styled(
                    &msg.message,
                    Style::default()
                        .bg(self.theme.flash_warn_bg)
                        .fg(self.theme.flash_warn_fg)
                        .add_modifier(Modifier::BOLD),
                ),
                MsgType::Info => Span::styled(
                    &msg.message,
                    Style::default()
                        .bg(self.theme.flash_info_bg)
                        .fg(self.theme.flash_info_fg)
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

#[derive(PartialEq, Eq, Clone)]
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
