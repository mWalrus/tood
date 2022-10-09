use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct ToodKeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl ToodKeyEvent {
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

impl ToString for ToodKeyEvent {
    fn to_string(&self) -> String {
        let key = match self.code {
            KeyCode::Char(c) if c == ' ' => '˽',
            KeyCode::Char(c) => c,
            KeyCode::Tab | KeyCode::BackTab => '⇥',
            KeyCode::Esc => '⎋',
            KeyCode::Enter => '⏎',
            _ => '#',
        };
        match self.modifiers {
            KeyModifiers::SHIFT => format!("⇪{}", key.to_uppercase()),
            KeyModifiers::CONTROL => format!("^{key}"),
            _ => key.to_string(),
        }
    }
}

pub fn key_match(ev: &KeyEvent, binding: &ToodKeyEvent) -> bool {
    ev.code == binding.code && ev.modifiers == binding.modifiers
}

impl PartialEq for ToodKeyEvent {
    fn eq(&self, other: &Self) -> bool {
        let ev: KeyEvent = self.into();
        let other: KeyEvent = other.into();
        ev == other
    }
}

impl From<&ToodKeyEvent> for KeyEvent {
    fn from(other: &ToodKeyEvent) -> Self {
        Self::new(other.code, other.modifiers)
    }
}

pub struct ToodKeyList {
    pub move_up: ToodKeyEvent,
    pub move_down: ToodKeyEvent,
    pub secondary_move_up: ToodKeyEvent,
    pub secondary_move_down: ToodKeyEvent,
    pub toggle_completed: ToodKeyEvent,
    pub add_todo: ToodKeyEvent,
    pub add_description: ToodKeyEvent,
    pub edit_todo: ToodKeyEvent,
    pub remove_todo: ToodKeyEvent,
    pub mark_recurring: ToodKeyEvent,
    pub submit: ToodKeyEvent,
    pub find: ToodKeyEvent,
    pub back: ToodKeyEvent,
    pub quit: ToodKeyEvent,
}

#[rustfmt::skip]
impl Default for ToodKeyList {
    fn default() -> Self {
        Self {
            move_up:             ToodKeyEvent::new(KeyCode::Char('k'), KeyModifiers::empty()),
            move_down:           ToodKeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty()),
            secondary_move_up:   ToodKeyEvent::new(KeyCode::BackTab,   KeyModifiers::SHIFT),
            secondary_move_down: ToodKeyEvent::new(KeyCode::Tab,       KeyModifiers::empty()),
            toggle_completed:    ToodKeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()),
            add_todo:            ToodKeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty()),
            add_description:     ToodKeyEvent::new(KeyCode::Char('e'), KeyModifiers::CONTROL),
            edit_todo:           ToodKeyEvent::new(KeyCode::Char('e'), KeyModifiers::empty()),
            remove_todo:         ToodKeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty()),
            mark_recurring:      ToodKeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
            submit:              ToodKeyEvent::new(KeyCode::Enter,     KeyModifiers::empty()),
            find:                ToodKeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty()),
            back:                ToodKeyEvent::new(KeyCode::Esc,       KeyModifiers::empty()),
            quit:                ToodKeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty()),
        }
    }
}
