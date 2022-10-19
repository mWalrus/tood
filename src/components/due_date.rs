use anyhow::Result;
use crossterm::event::KeyEvent;
use kanal::Sender;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::Clear,
    Frame,
};

use crate::{
    app::{AppMessage, State},
    keys::{key_match, keymap::SharedKeyList},
    widgets::{
        calendar::{Calendar, CalendarState},
        time_picker::{TimePicker, TimePickerState},
    },
};

use super::{utils, Component};

// FIXME: move to due_date component
// FIXME: implement From<NaiveDateTime> for ListState
pub struct DueDateComponent {
    pub calendar: Calendar,
    pub calendar_state: CalendarState,
    pub time_picker: TimePicker,
    pub time_picker_state: TimePickerState,
    focused_widget: DueDateWidgetHasFocus,
    keys: SharedKeyList,
    message_tx: Sender<AppMessage>,
}

enum DueDateWidgetHasFocus {
    Cal,
    Time,
}

impl DueDateComponent {
    pub fn new(keys: SharedKeyList, message_tx: Sender<AppMessage>) -> Self {
        let calendar = Calendar::default();
        let num_days = calendar.num_days();
        Self {
            calendar,
            calendar_state: CalendarState::new(num_days),
            time_picker: TimePicker::default(),
            time_picker_state: TimePickerState::with_current_time(),
            focused_widget: DueDateWidgetHasFocus::Cal,
            keys,
            message_tx,
        }
    }
}

impl Component for DueDateComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let calendar_rect = utils::calendar_rect(f.size());
        let picker_rect = Rect {
            height: 3,
            y: calendar_rect.y + calendar_rect.height,
            ..calendar_rect
        };
        f.render_widget(Clear, calendar_rect);
        f.render_widget(Clear, picker_rect);

        let cal_block = utils::default_block("Calendar");
        let time_block = utils::default_block("Time picker");

        match self.focused_widget {
            DueDateWidgetHasFocus::Cal => {
                self.calendar
                    .block(cal_block.border_style(Style::default().fg(Color::Blue)));
                self.time_picker.block(time_block);
            }
            DueDateWidgetHasFocus::Time => {
                self.calendar.block(cal_block);
                self.time_picker
                    .block(time_block.border_style(Style::default().fg(Color::Blue)));
            }
        }

        // FIXME: avoid clone
        f.render_stateful_widget(
            self.calendar.clone(),
            calendar_rect,
            &mut self.calendar_state,
        );
        f.render_stateful_widget(
            self.time_picker.clone(),
            picker_rect,
            &mut self.time_picker_state,
        )
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        match self.focused_widget {
            DueDateWidgetHasFocus::Cal => {
                if key_match(&key, &self.keys.move_up) {
                    self.calendar_state.up();
                } else if key_match(&key, &self.keys.move_down) {
                    self.calendar_state.down();
                } else if key_match(&key, &self.keys.move_left) {
                    self.calendar_state.left();
                } else if key_match(&key, &self.keys.move_right) {
                    self.calendar_state.right();
                } else if key_match(&key, &self.keys.alt_move_down) {
                    self.focused_widget = DueDateWidgetHasFocus::Time;
                }
            }
            DueDateWidgetHasFocus::Time => {
                if key_match(&key, &self.keys.move_up) {
                    self.time_picker_state.prev();
                } else if key_match(&key, &self.keys.move_down) {
                    self.time_picker_state.next();
                } else if key_match(&key, &self.keys.move_left) {
                    self.time_picker_state.toggle_focus();
                } else if key_match(&key, &self.keys.move_right) {
                    self.time_picker_state.toggle_focus();
                } else if key_match(&key, &self.keys.alt_move_down) {
                    self.focused_widget = DueDateWidgetHasFocus::Cal;
                }
            }
        }
        // this should always be handled no matter the focus
        if key_match(&key, &self.keys.back) {
            // set to AddTodo since it just changes the state
            // while EditTodo copies the currently selected todo's
            // contents into the edit view fields
            self.message_tx
                .send(AppMessage::InputState(State::AddTodo))?;
        }
        Ok(())
    }
}
