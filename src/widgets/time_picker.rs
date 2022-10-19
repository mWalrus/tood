use chrono::{Local, Timelike};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, ListState, StatefulWidget, Widget},
};

use crate::components::utils;

struct PickerFocusState(PickerFocus);

#[derive(PartialEq, Eq)]
enum PickerFocus {
    Hour,
    Minute,
}

impl PickerFocusState {
    fn toggle(&mut self) {
        if self.0 == PickerFocus::Hour {
            self.0 = PickerFocus::Minute;
        } else {
            self.0 = PickerFocus::Hour;
        }
    }
}

pub struct TimePickerState {
    hour_state: ListState,
    minute_state: ListState,
    focus_state: PickerFocusState,
}

impl TimePickerState {
    pub fn with_current_time() -> Self {
        let now = Local::now().time();
        let hour = now.hour() as usize;
        let minute = now.minute() as usize;

        let mut hour_state = ListState::default();
        hour_state.select(Some(hour));
        let mut minute_state = ListState::default();
        minute_state.select(Some(minute));

        Self {
            hour_state,
            minute_state,
            focus_state: PickerFocusState(PickerFocus::Hour),
        }
    }

    pub fn toggle_focus(&mut self) {
        self.focus_state.toggle();
    }

    pub fn next(&mut self) {
        match self.focus_state.0 {
            PickerFocus::Hour => {
                let i = if let Some(s) = self.hour_state.selected() {
                    if s + 1 > 23 {
                        0
                    } else {
                        s + 1
                    }
                } else {
                    unreachable!()
                };
                self.hour_state.select(Some(i));
            }
            PickerFocus::Minute => {
                let i = if let Some(s) = self.minute_state.selected() {
                    if s + 1 > 59 {
                        0
                    } else {
                        s + 1
                    }
                } else {
                    unreachable!() // we init the state with selections
                };
                self.minute_state.select(Some(i));
            }
        }
    }

    pub fn prev(&mut self) {
        match self.focus_state.0 {
            PickerFocus::Hour => {
                let i = if let Some(s) = self.hour_state.selected() {
                    if s.checked_sub(1).is_none() {
                        23
                    } else {
                        s - 1
                    }
                } else {
                    unreachable!()
                };
                self.hour_state.select(Some(i));
            }
            PickerFocus::Minute => {
                let i = if let Some(s) = self.minute_state.selected() {
                    if s.checked_sub(1).is_none() {
                        59
                    } else {
                        s - 1
                    }
                } else {
                    unreachable!() // we init the state with selections
                };
                self.minute_state.select(Some(i));
            }
        }
    }
}

#[derive(Clone)]
pub struct TimePicker {
    style: Style,
    selected_style: Style,
    block: Block<'static>,
}

impl Default for TimePicker {
    fn default() -> Self {
        Self {
            style: Style::default(),
            selected_style: Style::default()
                .bg(Color::Indexed(8))
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
            block: utils::default_block("Time picker"),
        }
    }
}

impl TimePicker {
    pub fn block(&mut self, block: Block<'static>) {
        self.block = block;
    }
}

impl StatefulWidget for TimePicker {
    type State = TimePickerState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);

        let picker_area = self.block.inner(area);
        let mid_of_area = picker_area.x + picker_area.width / 2;

        let hour_x = mid_of_area.saturating_sub(2);
        let minute_x = mid_of_area + 1;

        self.block.render(area, buf);

        // abort render if size is too small
        if picker_area.width < 1 || picker_area.height < 1 {
            return;
        }

        // TODO: 12h time format
        // safe to unwrap since we init the `TimePickerState` with selections
        let (hour, minute) = (
            format!("{:0>2}", state.hour_state.selected().unwrap()),
            format!("{:0>2}", state.minute_state.selected().unwrap()),
        );

        let (hour_style, minute_style) = match state.focus_state.0 {
            PickerFocus::Hour => (self.selected_style, self.style),
            PickerFocus::Minute => (self.style, self.selected_style),
        };

        buf.set_string(hour_x, picker_area.y, hour, hour_style);
        buf.set_string(mid_of_area, picker_area.y, ":", self.style);
        buf.set_string(minute_x, picker_area.y, minute, minute_style);
    }
}
