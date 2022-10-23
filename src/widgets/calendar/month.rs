use chrono::{Datelike, Local, NaiveDate, Weekday};

pub static NAMES: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub static MONTH_COUNT: usize = 6;

#[derive(Debug, Clone, Copy)]
pub struct Month {
    pub name: &'static str,
    pub num: u32,
    pub year: i32,
    pub days: u8,
    pub padding: u8,
}

impl Month {
    pub fn new(y: i32, m: u32, days: u8) -> Self {
        let d = NaiveDate::from_ymd(y, m, 1);

        let first_monday = d.week(Weekday::Mon).first_day();
        let days_since_monday = d.signed_duration_since(first_monday).num_days();

        let name = NAMES[m as usize - 1];

        Self {
            name,
            num: m,
            year: d.year(),
            days,
            padding: days_since_monday as u8,
        }
    }

    pub fn ym(&self) -> (i32, u32) {
        (self.year, self.num)
    }

    pub fn default_day(&self) -> usize {
        let now = Local::now();
        // give back current date if the selected
        // month is the current active month of the year
        if self.num == now.month() {
            return now.day() as usize;
        }
        0
    }

    pub fn num_days(&self) -> usize {
        self.days as usize
    }
}
