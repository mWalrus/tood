use chrono::{Datelike, Local, NaiveDate, Weekday};
use num_traits::FromPrimitive;

#[derive(Debug, Clone)]
pub struct Month {
    pub name: &'static str,
    pub num: u32,
    pub year: i32,
    pub days: u8,
    pub first_week_padding: u8,
}

impl Month {
    pub fn new(y: i32, m: u32, days: u8) -> Self {
        let d = NaiveDate::from_ymd(y, m, 1);

        // the first day of the month
        let first_of_the_month = NaiveDate::from_ymd(d.year(), m, 1);
        // the number of days since monday the first day of the month landed on
        let days_since_monday = first_of_the_month
            .signed_duration_since(first_of_the_month.week(Weekday::Mon).first_day())
            .num_days();

        let name = chrono::Month::from_u32(m).unwrap().name();

        Self {
            name,
            num: m,
            year: d.year(),
            days,
            first_week_padding: days_since_monday as u8,
        }
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn default_day(&self) -> usize {
        let now = Local::now();
        if self.num == now.month() {
            return now.day() as usize;
        }
        0
    }

    pub fn num_days(&self) -> usize {
        self.days as usize
    }

    pub fn num(&self) -> u32 {
        self.num
    }
}
