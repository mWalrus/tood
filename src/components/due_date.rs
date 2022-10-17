use anyhow::Result;
use crossterm::event::KeyEvent;
use tui::{backend::Backend, widgets::Clear, Frame};

use crate::{
    keys::{key_match, keymap::SharedKeyList},
    widgets::calendar::{Calendar, CalendarState},
};

use super::{utils, Component};

// FIXME: move to due_date component
// FIXME: implement From<NaiveDateTime> for ListState
pub struct DueDateComponent {
    pub calendar: Calendar,
    pub calendar_state: CalendarState,
    calendar_is_visible: bool,
    keys: SharedKeyList,
}

impl DueDateComponent {
    pub fn new(keys: SharedKeyList) -> Self {
        Self {
            calendar: Calendar::default(),
            calendar_state: CalendarState::default(),
            calendar_is_visible: true,
            keys,
        }
    }
    pub fn cal_right(&mut self) {
        let i = match self.calendar_state.selected() {
            Some(i) => {
                if i >= self.calendar.cells.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.calendar_state.select(Some(i));
    }

    pub fn cal_left(&mut self) {
        let i = match self.calendar_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.calendar.cells.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.calendar_state.select(Some(i));
    }

    pub fn cal_down(&mut self) {
        let i = match self.calendar_state.selected() {
            Some(i) => {
                if i + 7 >= self.calendar.cells.len() - 1 {
                    self.calendar.cells.len() - 1
                } else {
                    i + 7
                }
            }
            None => 0,
        };
        self.calendar_state.select(Some(i));
    }

    pub fn cal_up(&mut self) {
        let i = match self.calendar_state.selected() {
            Some(i) => {
                if i < 7 {
                    0
                } else {
                    i - 7
                }
            }
            None => 0,
        };
        self.calendar_state.select(Some(i));
    }
}

impl Component for DueDateComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let rect = utils::calendar_rect(f.size());
        f.render_widget(Clear, rect);
        // FIXME: avoid clone
        f.render_stateful_widget(self.calendar.clone(), rect, &mut self.calendar_state);
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        if self.calendar_is_visible {
            if key_match(&key, &self.keys.move_up) {
                self.cal_up();
            } else if key_match(&key, &self.keys.move_down) {
                self.cal_down();
            } else if key_match(&key, &self.keys.move_left) {
                self.cal_left();
            } else if key_match(&key, &self.keys.move_right) {
                self.cal_right();
            }
            // TODO: more binds
        }
        Ok(())
    }
}
