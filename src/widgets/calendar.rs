use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, Weekday};
use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, Borders, Cell as TableCell, Clear, ListState, Row, StatefulWidget, Table,
        TableState, Widget,
    },
    Frame,
};

use crate::components::{utils, Component};

#[derive(Debug, Clone)]
pub struct Cell {
    date: Option<u32>,
    is_today: bool,
}

impl Cell {
    fn empty() -> Self {
        Cell {
            date: None,
            is_today: false,
        }
    }

    fn with_date(d: u32, is_today: bool) -> Self {
        Cell {
            date: Some(d),
            is_today,
        }
    }
}

// FIXME: add method `with_selected_date` which can be used when editing
//        an existing todo

#[derive(Debug, Clone)]
pub struct Calendar {
    pub state: ListState,
    pub cells: Vec<Cell>,
    pub is_visible: bool,
    block: Option<Block<'static>>,
    style: Style,
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
            .num_days()
            + 1; // account for current day as well

        // pad calendar with cells if monday isn't the first day of the month
        if days_since_monday > 0 {
            for _ in 0..days_since_monday {
                cells.push(Cell::empty());
            }
        }

        let mut nth_day_of_the_month = first_of_the_month;

        while nth_day_of_the_month.month() == month {
            let nth_day = nth_day_of_the_month.day();
            cells.push(Cell::with_date(
                nth_day_of_the_month.day(),
                nth_day == d.day(),
            ));
            nth_day_of_the_month += one_day;
        }

        let mut state = ListState::default();
        state.select(Some(d.day0() as usize));

        Self {
            state,
            cells,
            is_visible: false,
            block: Some(utils::default_block("Calendar")),
            style: Style::default(),
        }
    }
}

impl StatefulWidget for Calendar {
    type State = ListState;
    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let calendar_area = match self.block.take() {
            Some(b) => {
                // get the inner area
                let inner_area = b.inner(area);
                // render the block (borders, title, etc)
                b.render(area, buf);
                // return
                inner_area
            }
            None => area,
        };

        // abort render if size is too small
        if calendar_area.width < 1 || calendar_area.height < 1 {
            return;
        }

        // we do not need account for offsets size since the entirity
        // of the calendar will be rendered in the area.
        // ...

        let (mut offset_x, mut offset_y) = (calendar_area.x, calendar_area.y);

        // divide by eight since we want 7 equally sized cells to render the date inside of
        let cell_width = calendar_area.width / 7;

        // render each cell
        for mut cell in self.cells.into_iter() {
            let cell_text = match cell.date.take() {
                Some(d) => format!("{d:>2}"),
                None => String::from("  "),
            };

            // define a cell area which we can use to render the number
            let cell_area = Rect {
                x: offset_x,
                y: offset_y,
                width: cell_width,
                height: 1,
            };

            let cell_style = if cell.is_today {
                Style::default()
                    .bg(Color::Indexed(8))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            buf.set_style(cell_area, cell_style);

            // render the date number in the center of the current cell
            let center_x = cell_area.x + (cell_area.width / 2) - 1;
            buf.set_string(center_x, cell_area.y, cell_text, cell_style);

            // check if we are treading boundaries
            if offset_x + cell_width >= calendar_area.width {
                offset_y += 2;
                offset_x = calendar_area.x;
            } else {
                offset_x += cell_width;
            }
        }
        buf.set_string(
            calendar_area.x,
            calendar_area.y,
            format!(
                "x: {}, y: {}, width: {}, height: {}, cell_width: {}",
                calendar_area.x,
                calendar_area.y,
                calendar_area.width,
                calendar_area.height,
                cell_width
            ),
            Style::default(),
        );
    }
}

impl Calendar {
    pub fn toggle_visible(&mut self) {
        self.is_visible = !self.is_visible;
    }
}
