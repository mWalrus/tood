use anyhow::Result;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use crossterm::event::KeyEvent;
use kanal::Sender;
use std::error::Error;
use tui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Clear},
    Frame,
};
use tui_utils::{component::Component, keys::key_match};

use crate::{
    app::{AppMessage, AppState},
    keys::keymap::SharedKeyList,
    theme::theme::SharedTheme,
    widgets::{
        calendar::{Calendar, CalendarState},
        time_picker::{TimePicker, TimePickerState},
    },
};

use super::{notification::FlashMsg, utils};

pub struct DueDateComponent {
    pub calendar: Calendar,
    pub calendar_state: CalendarState,
    pub time_picker: TimePicker,
    pub time_picker_state: TimePickerState,
    focused_widget: DueDateWidgetHasFocus,
    keys: SharedKeyList,
    theme: SharedTheme,
    flash_tx: Sender<FlashMsg>,
}

enum DueDateWidgetHasFocus {
    Cal,
    Time,
}

impl DueDateComponent {
    pub fn new(keys: SharedKeyList, theme: SharedTheme, flash_tx: Sender<FlashMsg>) -> Self {
        let calendar = Calendar::default();

        let (day, num_days) = if let Some(month) = calendar.current_month() {
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
            theme,
            flash_tx,
        }
    }

    pub fn get_date_time(&self) -> NaiveDateTime {
        let month_index = self.calendar_state.selected_month();
        let month = self.calendar.get_month_by_index(month_index).unwrap();
        let (y, mo) = month.ym();
        let d = self.calendar_state.selected_day();

        let (h, mi) = self.time_picker_state.hour_minute();
        // FIXME: handle OOB failure
        let date = NaiveDate::from_ymd_opt(y, mo, d).unwrap();
        let time = NaiveTime::from_hms_opt(h, mi, 0).unwrap();
        NaiveDateTime::new(date, time)
    }

    pub fn reset_date_time(&mut self) -> Result<()> {
        if let Some(current_month) = self.calendar.current_month() {
            let today = current_month.default_day();
            if let Err(e) = self.calendar_state.set_date(today) {
                self.flash_tx.send(FlashMsg::err(e))?;
            }
            self.time_picker_state = TimePickerState::with_current_time();
        }
        Ok(())
    }

    pub fn set_date_time(&mut self, dt: NaiveDateTime) -> Result<()> {
        let date = dt.date();
        let month = date.month0();

        if let Some((i, m)) = self.calendar.get_month_and_index_by_num(month as usize) {
            let day = date.day0();
            let num_days = m.num_days();

            let time = dt.time();
            let hour = time.hour();
            let minute = time.minute();

            self.time_picker_state = TimePickerState::with_hm(hour, minute);
            match CalendarState::with_date(i, day as usize, num_days) {
                Ok(state) => self.calendar_state = state,
                Err(e) => self.flash_tx.send(FlashMsg::err(e))?,
            }
        }
        Ok(())
    }
}

impl Component for DueDateComponent {
    type Message = AppMessage;

    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, _dim: bool) {
        let calendar_rect = utils::calendar_rect(f.size());
        let picker_rect = Rect {
            height: 3,
            y: calendar_rect.y + calendar_rect.height,
            ..calendar_rect
        };
        f.render_widget(Clear, calendar_rect);
        f.render_widget(Clear, picker_rect);

        let cal_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title("Calendar");
        let time_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title("Time picker");

        match self.focused_widget {
            DueDateWidgetHasFocus::Cal => {
                self.calendar.block(
                    cal_block.border_style(Style::default().fg(self.theme.move_mode_border)),
                );
                self.time_picker.block(time_block);
            }
            DueDateWidgetHasFocus::Time => {
                self.calendar.block(cal_block);
                self.time_picker.block(
                    time_block.border_style(Style::default().fg(self.theme.move_mode_border)),
                );
            }
        }

        // FIXME: avoid clone
        //        idk if we can avoid it since we cannot derive copy on block
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

    fn handle_input(&mut self, key: KeyEvent) -> Result<Self::Message, Box<dyn Error>> {
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
                } else if key_match(&key, &self.keys.move_left)
                    || key_match(&key, &self.keys.move_right)
                {
                    self.time_picker_state.toggle_focus();
                } else if key_match(&key, &self.keys.alt_move_down) {
                    self.focused_widget = DueDateWidgetHasFocus::Cal;
                }
            }
        }
        // this should always be handled no matter the focus
        if key_match(&key, &self.keys.back) {
            self.reset_date_time()?;
            // set to AddTodo since it just changes the state
            // while EditTodo copies the currently selected todo's
            // contents into the edit view fields
            return Ok(AppMessage::InputState(AppState::AddTodo));
        } else if key_match(&key, &self.keys.submit) {
            let date_time = self.get_date_time();
            self.reset_date_time()?;
            return Ok(AppMessage::SetDueDate(date_time));
        }
        Ok(AppMessage::NoAction)
    }
}
