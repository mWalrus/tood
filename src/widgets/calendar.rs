use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, ListState, StatefulWidget, Widget},
};

use crate::components::utils;

#[derive(Debug, Clone)]
pub struct Cell {
    date: u32,
    is_today: bool,
}

impl Cell {
    fn with_date(date: u32, is_today: bool) -> Self {
        Cell { date, is_today }
    }
}

pub type CalendarState = ListState;

// FIXME: add method `with_selected_date` which can be used when editing
//        an existing todo

#[derive(Debug, Clone)]
pub struct Calendar {
    pub empty_days: usize,
    pub cells: Vec<Cell>,
    pub is_visible: bool,
    block: Block<'static>,
    style: Style,
}

impl Calendar {
    pub fn toggle_visible(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for Calendar {
    fn default() -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        let dt = Local::now().naive_local();

        let d = dt.date();
        let _t = dt.time();

        let _week = d.week(Weekday::Mon);
        let month = d.month();

        // the first day of the month
        let first_of_the_month = NaiveDate::from_ymd(d.year(), month, 1);
        // A day in duration (used for adding dates to the calendar)
        let one_day = Duration::hours(24);
        // the number of days since monday the first day of the month landed on
        let days_since_monday = first_of_the_month
            .signed_duration_since(first_of_the_month.week(Weekday::Mon).first_day())
            .num_days();

        let mut nth_day_of_the_month = first_of_the_month;

        while nth_day_of_the_month.month() == month {
            let nth_day = nth_day_of_the_month.day();
            cells.push(Cell::with_date(
                nth_day_of_the_month.day(),
                nth_day == d.day(),
            ));
            nth_day_of_the_month += one_day;
        }

        Self {
            empty_days: days_since_monday as usize,
            cells,
            is_visible: false,
            block: utils::default_block("Calendar"),
            style: Style::default(),
        }
    }
}

impl StatefulWidget for Calendar {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        // get the inner area
        let calendar_area = self.block.inner(area);
        // render the block (borders, title, etc)
        self.block.render(area, buf);

        // abort render if size is too small
        if calendar_area.width < 1 || calendar_area.height < 1 {
            return;
        }

        // we do not need account for offsets size since the entirity
        // of the calendar will be rendered in the area.
        // ...

        // divide by eight since we want 7 equally sized cells to render the date inside of
        let cell_width = calendar_area.width / 7;
        let cell_mid = (cell_width / 2).saturating_sub(1);
        let cell_height = 2;

        // print header row
        for (i, day) in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"]
            .iter()
            .enumerate()
        {
            buf.set_string(
                calendar_area.x + i as u16 * cell_width + cell_mid,
                calendar_area.y,
                day,
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED),
            );
        }

        //                                                give space for header row ⬇
        let (mut offset_x, mut offset_y) = (calendar_area.x, calendar_area.y + cell_height);

        // pad for empty day cells
        for _ in 0..self.empty_days {
            buf.set_string(offset_x, offset_y, "    ", Style::default());
            offset_x += 4;
        }

        // render each cell
        for (i, cell) in self.cells.into_iter().enumerate() {
            let cell_text = format!("{:>2}", cell.date);

            // define a cell area which we can use to render the number
            let cell_area = Rect {
                x: offset_x,
                y: offset_y,
                width: cell_width,
                height: cell_height,
            };

            let cell_style = if let Some(s) = state.selected() {
                if s == i {
                    Style::default()
                        .bg(Color::Indexed(8))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                }
            } else if cell.is_today {
                // FIXME: move this to a better place
                // set the current state
                state.select(Some(i));
                Style::default()
                    .bg(Color::Indexed(8))
                    .add_modifier(Modifier::BOLD)
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

            // if offset_y > calendar_area.height {
            //     return;
            // }
        }
    }
}
