use super::keymap::ToodKeyList;
use crate::config::Config;
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use tui_utils::keys::Keybind;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct KeyConfig {
    pub move_up: Option<Keybind>,
    pub move_down: Option<Keybind>,
    pub move_left: Option<Keybind>,
    pub move_right: Option<Keybind>,
    pub alt_move_up: Option<Keybind>,
    pub alt_move_down: Option<Keybind>,
    pub alt_move_left: Option<Keybind>,
    pub alt_move_right: Option<Keybind>,
    pub toggle_completed: Option<Keybind>,
    pub add_todo: Option<Keybind>,
    pub external_editor: Option<Keybind>,
    pub edit_todo: Option<Keybind>,
    pub open_calendar: Option<Keybind>,
    pub remove_todo: Option<Keybind>,
    pub mark_recurring: Option<Keybind>,
    pub desc_scroll_up: Option<Keybind>,
    pub desc_scroll_down: Option<Keybind>,
    pub submit: Option<Keybind>,
    pub find_mode: Option<Keybind>,
    pub move_mode: Option<Keybind>,
    pub back: Option<Keybind>,
    pub quit: Option<Keybind>,
}

impl Config for KeyConfig {}

impl KeyConfig {
    pub fn read_from_file() -> Result<Option<Self>, ConfyError> {
        match confy::load("tood", Some("key-config")) {
            Ok(cfg) => Ok(Some(cfg)),
            Err(_) if Self::file_is_empty("key-config")? => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_shared_list(self) -> Rc<ToodKeyList> {
        let dkl = ToodKeyList::default();

        let list = ToodKeyList {
            move_up: self.move_up.unwrap_or(dkl.move_up),
            move_down: self.move_down.unwrap_or(dkl.move_down),
            move_left: self.move_left.unwrap_or(dkl.move_left),
            move_right: self.move_right.unwrap_or(dkl.move_right),
            alt_move_up: self.alt_move_up.unwrap_or(dkl.alt_move_up),
            alt_move_down: self.alt_move_down.unwrap_or(dkl.alt_move_down),
            alt_move_left: self.alt_move_left.unwrap_or(dkl.alt_move_left),
            alt_move_right: self.alt_move_right.unwrap_or(dkl.alt_move_right),
            toggle_completed: self.toggle_completed.unwrap_or(dkl.toggle_completed),
            add_todo: self.add_todo.unwrap_or(dkl.add_todo),
            external_editor: self.external_editor.unwrap_or(dkl.external_editor),
            edit_todo: self.edit_todo.unwrap_or(dkl.edit_todo),
            open_calendar: self.open_calendar.unwrap_or(dkl.open_calendar),
            remove_todo: self.remove_todo.unwrap_or(dkl.remove_todo),
            mark_recurring: self.mark_recurring.unwrap_or(dkl.mark_recurring),
            desc_scroll_up: self.desc_scroll_up.unwrap_or(dkl.desc_scroll_up),
            desc_scroll_down: self.desc_scroll_down.unwrap_or(dkl.desc_scroll_down),
            submit: self.submit.unwrap_or(dkl.submit),
            find_mode: self.find_mode.unwrap_or(dkl.find_mode),
            move_mode: self.move_mode.unwrap_or(dkl.move_mode),
            back: self.back.unwrap_or(dkl.back),
            quit: self.quit.unwrap_or(dkl.quit),
        };
        Rc::new(list)
    }
}
