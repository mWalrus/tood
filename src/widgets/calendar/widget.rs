use super::{month::MONTH_COUNT, CalendarState, Month};
use crate::components::utils;
use chrono::{Datelike, Local, NaiveDate};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, StatefulWidget, Widget},
};
use tui_utils::style::highlight_style;

#[derive(Debug, Clone)]
pub struct Calendar {
    months: Vec<Month>,
    block: Block<'static>,
    style: Style,
}

impl Default for Calendar {
    fn default() -> Self {
        let mut months = Vec::with_capacity(MONTH_COUNT);
        let now = Local::now().date_naive();
        // FIXME: handle OOB unwrap
        let mut current_month = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();

        for _ in 0..MONTH_COUNT {
            let year = current_month.year();
            let month = current_month.month();
            let (next_month_year, next_month) = if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            };
            // FIXME: handle OOB unwrap
            let month_duration = NaiveDate::from_ymd_opt(next_month_year, next_month, 1)
                .unwrap()
                .signed_duration_since(current_month);

            months.push(Month::new(year, month, month_duration.num_days() as u8));

            current_month += month_duration;
        }

        Self {
            months,
            block: utils::default_block("Calendar"),
            style: Style::default(),
        }
    }
}

impl Calendar {
    pub fn block(&mut self, block: Block<'static>) {
        self.block = block;
    }

    pub fn get_month_and_index_by_num(&self, month_num: usize) -> Option<(usize, &Month)> {
        self.months
            .iter()
            .enumerate()
            .find(|m| m.1.num == month_num as u32)
    }

    pub fn get_month_by_index(&self, i: usize) -> Option<&Month> {
        if i > self.months.len() {
            return None;
        }
        Some(&self.months[i])
    }

    pub fn current_month(&self) -> Option<&Month> {
        if self.months.is_empty() {
            return None;
        }
        Some(&self.months[0])
    }
}

impl StatefulWidget for Calendar {
    type State = CalendarState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let month_i = state.selected_month;

        let (header, padding, num_days) = {
            let m = self.get_month_by_index(month_i).unwrap();
            (m.formatted_month_year(), m.padding, m.num_days())
        };

        buf.set_style(area, self.style);
        // get the inner area
        let calendar_area = self.block.inner(area);
        // render the block (borders, title, etc)
        self.block.render(area, buf);

        // abort render if size is too small
        if calendar_area.width < 1 || calendar_area.height < 1 {
            return;
        }

        let header_x_mid = calendar_area.x + (calendar_area.width / 2);
        let month_header_x = header_x_mid - header.len() as u16 / 2;

        buf.set_string(
            month_header_x,
            calendar_area.y,
            header,
            Style::default().add_modifier(Modifier::BOLD),
        );

        let cell_width = calendar_area.width / 7;
        let cell_mid = (cell_width / 2).saturating_sub(1);
        let cell_height = 2;

        // print day row
        for (i, day) in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"]
            .iter()
            .enumerate()
        {
            buf.set_string(
                calendar_area.x + i as u16 * cell_width + cell_mid,
                calendar_area.y + cell_height,
                day,
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED),
            );
        }

        //                                                give space for header rows ⬇
        let (mut offset_x, mut offset_y) = (calendar_area.x, calendar_area.y + (cell_height * 2));

        // pad for empty day cells
        for _ in 0..padding {
            buf.set_string(offset_x, offset_y, "    ", Style::default());
            offset_x += 4;
        }

        // render each day
        for d in 1..=num_days {
            let cell_text = format!("{:>2}", d);

            // define a cell area which we can use to render the number
            let cell_area = Rect {
                x: offset_x,
                y: offset_y,
                width: cell_width,
                height: cell_height,
            };

            let cell_style = if d == state.selected_day {
                highlight_style()
            } else {
                Style::default()
            };

            buf.set_style(cell_area, cell_style);

            // render the date number in the center of the current cell
            buf.set_string(cell_area.x + cell_mid, cell_area.y, cell_text, cell_style);

            // check if we are treading boundaries
            if offset_x + cell_width >= calendar_area.x + calendar_area.width {
                offset_y += cell_area.height;
                offset_x = calendar_area.x;
            } else {
                offset_x += cell_width;
            }
        }
    }
}
