use super::keymap::ToodKeyList;
use crate::config::Config;
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

impl Config for KeyConfig {
    type Item = ToodKeyList;

    fn to_shared(self) -> Rc<ToodKeyList> {
        let dkl = ToodKeyList::default();

        #[rustfmt::skip]
        let list = ToodKeyList {
            move_up:           either!(self.move_up,          dkl.move_up),
            move_down:         either!(self.move_down,        dkl.move_down),
            move_left:         either!(self.move_left,        dkl.move_left),
            move_right:        either!(self.move_right,       dkl.move_right),
            alt_move_up:       either!(self.alt_move_up,      dkl.alt_move_up),
            alt_move_down:     either!(self.alt_move_down,    dkl.alt_move_down),
            alt_move_left:     either!(self.alt_move_left,    dkl.alt_move_left),
            alt_move_right:    either!(self.alt_move_right,   dkl.alt_move_right),
            toggle_completed:  either!(self.toggle_completed, dkl.toggle_completed),
            add_todo:          either!(self.add_todo,         dkl.add_todo),
            external_editor:   either!(self.external_editor,  dkl.external_editor),
            edit_todo:         either!(self.edit_todo,        dkl.edit_todo),
            open_calendar:     either!(self.open_calendar,    dkl.open_calendar),
            remove_todo:       either!(self.remove_todo,      dkl.remove_todo),
            mark_recurring:    either!(self.mark_recurring,   dkl.mark_recurring),
            desc_scroll_up:    either!(self.desc_scroll_up,   dkl.desc_scroll_up),
            desc_scroll_down:  either!(self.desc_scroll_down, dkl.desc_scroll_down),
            submit:            either!(self.submit,           dkl.submit),
            find_mode:         either!(self.find_mode,        dkl.find_mode),
            move_mode:         either!(self.move_mode,        dkl.move_mode),
            back:              either!(self.back,             dkl.back),
            quit:              either!(self.quit,             dkl.quit),
        };
        Rc::new(list)
    }
}
