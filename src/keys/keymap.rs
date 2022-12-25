use std::rc::Rc;

use crossterm::event::{KeyCode, KeyModifiers};
use tui_utils::keys::{Keybind, Shared};

use super::key_config::KeyConfig;
pub type SharedKeyList = Rc<ToodKeyList>;

#[derive(Debug, Shared)]
pub struct ToodKeyList {
    pub move_up: Keybind,
    pub move_down: Keybind,
    pub move_left: Keybind,
    pub move_right: Keybind,
    pub alt_move_up: Keybind,
    pub alt_move_down: Keybind,
    pub alt_move_left: Keybind,
    pub alt_move_right: Keybind,
    pub toggle_completed: Keybind,
    pub add_todo: Keybind,
    pub external_editor: Keybind,
    pub edit_todo: Keybind,
    pub open_calendar: Keybind,
    pub remove_todo: Keybind,
    pub mark_recurring: Keybind,
    pub submit: Keybind,
    pub find_mode: Keybind,
    pub move_mode: Keybind,
    pub back: Keybind,
    pub quit: Keybind,
}

#[rustfmt::skip]
impl Default for ToodKeyList {
    fn default() -> Self {
       Self {
            move_up:             Keybind::new(KeyCode::Up,        KeyModifiers::empty()),
            move_down:           Keybind::new(KeyCode::Down,      KeyModifiers::empty()),
            move_left:           Keybind::new(KeyCode::Left,      KeyModifiers::empty()),
            move_right:          Keybind::new(KeyCode::Right,     KeyModifiers::empty()),
            alt_move_up:         Keybind::new(KeyCode::BackTab,   KeyModifiers::SHIFT),
            alt_move_down:       Keybind::new(KeyCode::Tab,       KeyModifiers::empty()),
            alt_move_left:       Keybind::new(KeyCode::Char('H'), KeyModifiers::SHIFT),
            alt_move_right:      Keybind::new(KeyCode::Char('L'), KeyModifiers::SHIFT),
            toggle_completed:    Keybind::new(KeyCode::Char(' '), KeyModifiers::empty()),
            add_todo:            Keybind::new(KeyCode::Char('a'), KeyModifiers::empty()),
            external_editor:     Keybind::new(KeyCode::Char('e'), KeyModifiers::CONTROL),
            edit_todo:           Keybind::new(KeyCode::Char('e'), KeyModifiers::empty()),
            open_calendar:       Keybind::new(KeyCode::Char('d'), KeyModifiers::CONTROL),
            remove_todo:         Keybind::new(KeyCode::Char('d'), KeyModifiers::empty()),
            mark_recurring:      Keybind::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
            submit:              Keybind::new(KeyCode::Enter,     KeyModifiers::empty()),
            find_mode:           Keybind::new(KeyCode::Char('f'), KeyModifiers::empty()),
            move_mode:           Keybind::new(KeyCode::Char('m'), KeyModifiers::empty()),
            back:                Keybind::new(KeyCode::Esc,       KeyModifiers::empty()),
            quit:                Keybind::new(KeyCode::Char('q'), KeyModifiers::empty()),
        }
    }
}

impl ToodKeyList {
    pub fn init() -> SharedKeyList {
        if let Ok(cfg) = KeyConfig::read_from_file() {
            cfg.to_shared_list()
        } else {
            Self::shared()
        }
    }
}
