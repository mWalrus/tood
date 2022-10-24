use super::month::MONTH_COUNT;

pub struct CalendarState {
    pub selected_month: usize,
    pub num_months: usize,
    pub selected_day: usize,
    pub num_days: usize,
}

impl CalendarState {
    pub fn new(day: usize, num_days: usize) -> Self {
        Self {
            selected_month: 0,
            num_months: MONTH_COUNT,
            selected_day: day,
            num_days,
        }
    }

    pub fn with_date(month: usize, day: usize, num_days: usize) -> Result<Self, &'static str> {
        if month > MONTH_COUNT || day > num_days {
            return Err("Invalid date");
        }
        Ok(Self {
            selected_month: month,
            num_months: MONTH_COUNT,
            selected_day: day,
            num_days,
        })
    }

    pub fn set_date(&mut self, day: usize) -> Result<(), &'static str> {
        if day > self.num_days {
            return Err("Failed to set date: out of bounds");
        }
        self.selected_day = day;
        Ok(())
    }

    #[inline(always)]
    pub fn selected_day(&self) -> u32 {
        self.selected_day as u32
    }

    #[inline(always)]
    pub fn set_num_days(&mut self, nd: usize) {
        self.selected_day = self.selected_day.min(nd);
        self.num_days = nd;
    }

    #[inline(always)]
    pub fn selected_month(&self) -> usize {
        self.selected_month
    }

    // ========== Controls ===========

    #[inline(always)]
    pub fn next_month(&mut self) {
        self.selected_month = (self.num_months - 1).min(self.selected_month + 1);
    }

    #[inline(always)]
    pub fn prev_month(&mut self) {
        self.selected_month = self.selected_month.saturating_sub(1);
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
}
