use confy::ConfyError;
use serde::{Deserialize, Serialize};

use super::keymap::{ToodKeyEvent, ToodKeyList};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct KeyConfig {
    pub move_up: Option<ToodKeyEvent>,
    pub move_down: Option<ToodKeyEvent>,
    pub alt_move_up: Option<ToodKeyEvent>,
    pub alt_move_down: Option<ToodKeyEvent>,
    pub toggle_completed: Option<ToodKeyEvent>,
    pub add_todo: Option<ToodKeyEvent>,
    pub add_description: Option<ToodKeyEvent>,
    pub edit_todo: Option<ToodKeyEvent>,
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
            alt_move_up: self.alt_move_up.unwrap_or(dkl.alt_move_up),
            alt_move_down: self.alt_move_down.unwrap_or(dkl.alt_move_down),
            toggle_completed: self.toggle_completed.unwrap_or(dkl.toggle_completed),
            add_todo: self.add_todo.unwrap_or(dkl.add_todo),
            add_description: self.add_description.unwrap_or(dkl.add_description),
            edit_todo: self.edit_todo.unwrap_or(dkl.edit_todo),
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