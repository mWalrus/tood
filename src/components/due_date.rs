use anyhow::Result;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
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

use super::{notification::FlashMsg, utils, Component};

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

        let month = calendar.current_month();
        let (day, num_days) = if let Some(month) = month {
            (month.default_day(), month.num_days())
        } else {
            (1, 31)
        };

        Self {
            calendar,
            calendar_state: CalendarState::new(day, num_days),
            time_picker: TimePicker::default(),
            time_picker_state: TimePickerState::with_current_time(),
            focused_widget: DueDateWidgetHasFocus::Cal,
            keys,
            message_tx,
        }
    }

    pub fn get_date_time(&self) -> NaiveDateTime {
        let month_index = self.calendar_state.selected_month();
        let month = self.calendar.get_month_by_index(month_index).unwrap();
        let year = month.year();
        let month_num = month.num();
        let day = self.calendar_state.day();

        let (h, mi) = self.time_picker_state.hour_minute();
        let date = NaiveDate::from_ymd(year, month_num, day);
        let time = NaiveTime::from_hms(h, mi, 0);
        NaiveDateTime::new(date, time)
    }

    pub fn reset_date_time(&mut self) {
        if let Some(current_month) = self.calendar.current_month() {
            let today = current_month.default_day();
            if let Err(e) = self.calendar_state.set_date(today as usize) {
                self.message_tx
                    .send(AppMessage::Flash(FlashMsg::err(e)))
                    .unwrap();
            }
            self.time_picker_state = TimePickerState::with_current_time();
        }
    }

    pub fn set_date_time(&mut self, dt: NaiveDateTime) {
        let date = dt.date();
        let month = date.month0();

        if let Some(m) = self.calendar.get_month_index_by_num(month as usize) {
            let month = self.calendar.get_month_by_index(m).unwrap();
            let day = date.day0();
            let num_days = month.num_days();

            let time = dt.time();

            let hour = time.hour();
            let minute = time.minute();

            self.calendar_state = CalendarState::with_date(m, day as usize, num_days);
            self.time_picker_state = TimePickerState::with_hm(hour, minute);
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
                } else if key_match(&key, &self.keys.alt_move_left) {
                    self.calendar_state.prev_month();
                    if let Some(m) = self
                        .calendar
                        .get_month_by_index(self.calendar_state.selected_month())
                    {
                        self.calendar_state.set_num_days(m.num_days());
                    }
                } else if key_match(&key, &self.keys.alt_move_right) {
                    self.calendar_state.next_month();
                    if let Some(m) = self
                        .calendar
                        .get_month_by_index(self.calendar_state.selected_month())
                    {
                        self.calendar_state.set_num_days(m.num_days());
                    }
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
            self.reset_date_time();
            // set to AddTodo since it just changes the state
            // while EditTodo copies the currently selected todo's
            // contents into the edit view fields
            self.message_tx
                .send(AppMessage::InputState(State::AddTodo))?;
        } else if key_match(&key, &self.keys.submit) {
            let date_time = self.get_date_time();
            self.reset_date_time();
            self.message_tx.send(AppMessage::SetDueDate(date_time))?;
        }
        Ok(())
    }
}
