use confy::ConfyError;
use serde::{Deserialize, Serialize};

use super::keymap::{ToodKeyEvent, ToodKeyList};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct KeyConfig {
    pub move_up: Option<ToodKeyEvent>,
    pub move_down: Option<ToodKeyEvent>,
    pub move_left: Option<ToodKeyEvent>,
    pub move_right: Option<ToodKeyEvent>,
    pub alt_move_up: Option<ToodKeyEvent>,
    pub alt_move_down: Option<ToodKeyEvent>,
    pub alt_move_left: Option<ToodKeyEvent>,
    pub alt_move_right: Option<ToodKeyEvent>,
    pub toggle_completed: Option<ToodKeyEvent>,
    pub add_todo: Option<ToodKeyEvent>,
    pub external_editor: Option<ToodKeyEvent>,
    pub edit_todo: Option<ToodKeyEvent>,
    pub open_calendar: Option<ToodKeyEvent>,
    pub remove_todo: Option<ToodKeyEvent>,
    pub mark_recurring: Option<ToodKeyEvent>,
    pub submit: Option<ToodKeyEvent>,
    pub find_mode: Option<ToodKeyEvent>,
    pub move_mode: Option<ToodKeyEvent>,
    pub back: Option<ToodKeyEvent>,
    pub quit: Option<ToodKeyEvent>,
}

impl KeyConfig {
    pub fn read_from_file() -> Result<Self, ConfyError> {
        confy::load("tood", Some("key-config"))
    }

    pub fn to_list(self) -> ToodKeyList {
        let dkl = ToodKeyList::default();

        ToodKeyList {
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
            submit: self.submit.unwrap_or(dkl.submit),
            find_mode: self.find_mode.unwrap_or(dkl.find_mode),
            move_mode: self.move_mode.unwrap_or(dkl.move_mode),
            back: self.back.unwrap_or(dkl.back),
            quit: self.quit.unwrap_or(dkl.quit),
        }
    }
}
