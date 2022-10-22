use anyhow::{anyhow, Result};

pub struct CalendarState {
    pub selected_month: usize,
    pub month_count: usize,
    pub selected_day: usize,
    pub num_days: usize,
}

impl CalendarState {
    pub fn new(day: usize, num_days: usize) -> Self {
        Self {
            selected_month: 0,
            month_count: 6,
            selected_day: day,
            num_days,
        }
    }

    pub fn with_date(month: usize, day: usize, num_days: usize) -> Self {
        Self {
            selected_month: month,
            month_count: 6,
            selected_day: day,
            num_days,
        }
    }

    #[inline(always)]
    pub fn right(&mut self) {
        if self.selected_day < self.num_days {
            self.selected_day += 1;
        }
    }

    #[inline(always)]
    pub fn left(&mut self) {
        self.selected_day = self.selected_day.saturating_sub(1);
    }

    #[inline(always)]
    pub fn down(&mut self) {
        self.selected_day = self.num_days.min(self.selected_day + 7);
    }

    #[inline(always)]
    pub fn up(&mut self) {
        self.selected_day = 1.max(self.selected_day.saturating_sub(7));
    }

    pub fn set_date(&mut self, day: usize) -> Result<()> {
        if day > self.num_days {
            return Err(anyhow!("Failed to set date: out of bounds"));
        }
        self.selected_day = day;
        Ok(())
    }

    #[inline(always)]
    pub fn day(&self) -> u32 {
        self.selected_day as u32
    }

    #[inline(always)]
    pub fn next_month(&mut self) {
        if self.selected_month < self.month_count - 1 {
            self.selected_month += 1;
        }

        // load a new month
        if self.month_count - self.selected_month == 1 {
            // send app message and ask for another month
        }
    }

    #[inline(always)]
    pub fn prev_month(&mut self) {
        if self.selected_month > 0 {
            self.selected_month -= 1;
        }
    }

    #[inline(always)]
    pub fn set_num_days(&mut self, nd: usize) {
        if self.selected_day > nd {
            self.selected_day = nd;
        }
        self.num_days = nd;
    }

    #[inline(always)]
    pub fn selected_month(&self) -> usize {
        self.selected_month
    }
}
