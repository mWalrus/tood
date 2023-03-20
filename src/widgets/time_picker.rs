use crate::components::utils;
use chrono::{Local, Timelike};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, StatefulWidget, Widget},
};
use tui_utils::style::highlight_style;

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
    hour_state: usize,
    minute_state: usize,
    focus_state: PickerFocusState,
}

impl TimePickerState {
    pub fn with_current_time() -> Self {
        let now = Local::now().time();
        let hour = now.hour() as usize;
        let minute = now.minute() as usize;

        Self {
            hour_state: hour,
            minute_state: minute,
            focus_state: PickerFocusState(PickerFocus::Hour),
        }
    }

    pub fn with_hm(h: u32, m: u32) -> Self {
        Self {
            hour_state: h as usize,
            minute_state: m as usize,
            focus_state: PickerFocusState(PickerFocus::Hour),
        }
    }

    pub fn toggle_focus(&mut self) {
        self.focus_state.toggle();
    }

    pub fn next(&mut self) {
        match self.focus_state.0 {
            PickerFocus::Hour => {
                if self.hour_state + 1 > 23 {
                    self.hour_state = 0;
                } else {
                    self.hour_state += 1;
                }
            }
            PickerFocus::Minute => {
                if self.minute_state + 1 > 59 {
                    self.minute_state = 0;
                } else {
                    self.minute_state += 1;
                }
            }
        }
    }

    pub fn prev(&mut self) {
        match self.focus_state.0 {
            PickerFocus::Hour => {
                if self.hour_state.checked_sub(1).is_none() {
                    self.hour_state = 23;
                } else {
                    self.hour_state -= 1;
                }
            }
            PickerFocus::Minute => {
                if self.minute_state.checked_sub(1).is_none() {
                    self.minute_state = 59;
                } else {
                    self.minute_state -= 1;
                }
            }
        }
    }

    pub fn hour_minute(&self) -> (u32, u32) {
        (self.hour_state as u32, self.minute_state as u32)
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
            selected_style: highlight_style().add_modifier(Modifier::UNDERLINED),
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
        let mid_of_area = (picker_area.x + picker_area.width / 2).saturating_sub(1);

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
            format!("{:0>2}", state.hour_state),
            format!("{:0>2}", state.minute_state),
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
