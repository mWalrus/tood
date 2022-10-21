use anyhow::{anyhow, Result};
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, StatefulWidget, Widget},
};

use crate::components::utils;

#[derive(Debug, Clone)]
pub struct Cell(u32);

impl Cell {
    fn with_date(date: u32) -> Self {
        Cell(date)
    }
}

pub struct CalendarState {
    year: usize,
    month: usize,
    selected: usize,
    upper_bounds: usize,
}

impl CalendarState {
    pub fn new(ymdn: YMDN) -> Self {
        let (y, m, d, n) = ymdn;
        Self {
            year: y as usize,
            month: m as usize,
            selected: d as usize,
            upper_bounds: n as usize - 1,
        }
    }

    #[inline(always)]
    pub fn right(&mut self) {
        if self.selected >= self.upper_bounds {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }

    #[inline(always)]
    pub fn left(&mut self) {
        self.selected = self.selected.checked_sub(1).unwrap_or(self.upper_bounds);
    }

    #[inline(always)]
    pub fn down(&mut self) {
        self.selected = self.upper_bounds.min(self.selected + 7);
    }

    #[inline(always)]
    pub fn up(&mut self) {
        self.selected = 0.max(self.selected - 7);
    }

    pub fn set_date(&mut self, day: usize) -> Result<()> {
        if day > self.upper_bounds {
            return Err(anyhow!("Failed to set date: out of bounds"));
        }
        self.selected = day;
        Ok(())
    }

    #[inline(always)]
    pub fn ymd(&self) -> (i32, u32, u32) {
        (
            self.year as i32,
            self.month as u32,
            self.selected as u32 + 1,
        )
    }
}

pub type YMDN = (i32, u32, u32, u32);

// FIXME: add method `with_selected_date` which can be used when editing
//        an existing todo

#[derive(Debug, Clone)]
pub struct Calendar {
    pub empty_days: usize,
    pub cells: Vec<Cell>,
    pub is_visible: bool,
    ymdn: YMDN,
    block: Block<'static>,
    style: Style,
}

impl Calendar {
    pub fn today(&self) -> u32 {
        self.ymdn.2
    }
    pub fn block(&mut self, block: Block<'static>) {
        self.block = block;
    }

    pub fn ymdn(&self) -> YMDN {
        self.ymdn
    }
}

// TODO: move this implementation to `Month`.
//       my thought is that we can load in maybe 6 months worth of calendar
//       and using the state we can move through them for setting due dates.
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

            cells.push(Cell::with_date(nth_day));

            nth_day_of_the_month += one_day;
        }

        let ymdn = (dt.year(), month, d.day0(), cells.len() as u32);

        Self {
            empty_days: days_since_monday as usize,
            cells,
            is_visible: false,
            ymdn,
            block: utils::default_block("Calendar"),
            style: Style::default(),
        }
    }
}

impl StatefulWidget for Calendar {
    type State = CalendarState;
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
            let cell_text = format!("{:>2}", cell.0);

            // define a cell area which we can use to render the number
            let cell_area = Rect {
                x: offset_x,
                y: offset_y,
                width: cell_width,
                height: cell_height,
            };

            let cell_style = if i == state.selected {
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
