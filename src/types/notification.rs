use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

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
