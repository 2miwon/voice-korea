#![allow(non_snake_case)]
use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use dioxus_logger::tracing;
#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub today: Signal<(i32, u32, u32)>,
    pub year: Signal<i32>,
    pub month: Signal<u32>,
    pub day: Signal<u32>,
    pub weekday_of_month: Signal<chrono::Weekday>,
    pub last_weekday_of_month: Signal<chrono::Weekday>,
    pub last_date_of_month: Signal<u32>,
    pub last_date_of_prev_month: Signal<u32>,
    pub selected_date: Signal<(i32, u32, u32)>,

    pub selected_year: Signal<i32>,
    pub selected_month: Signal<u32>,
    pub selected_day: Signal<u32>,
}
impl Controller {
    pub fn init() -> Result<Self, RenderError> {
        let today = chrono::Local::now();
        let year = today.year();
        let month = today.month();
        let day = today.day();
        let weekday = NaiveDate::from_ymd_opt(year, month, 1).unwrap().weekday();
        let last_date_of_month = Self::find_last_date_of_month(year, month, None);
        let (next_year, next_month) = Self::next_month(year, month);
        let last_date_of_prev_month = Self::find_last_date_of_month(next_year, next_month, None);
        let last_weekday_of_month = NaiveDate::from_ymd_opt(year, month, last_date_of_month)
            .unwrap()
            .weekday();
        let ctrl = Self {
            today: use_signal(|| (year, month, day)),
            year: use_signal(|| year),
            month: use_signal(|| month),
            day: use_signal(|| day),
            weekday_of_month: use_signal(|| weekday),
            last_weekday_of_month: use_signal(|| last_weekday_of_month),
            last_date_of_month: use_signal(|| last_date_of_month),
            last_date_of_prev_month: use_signal(|| last_date_of_prev_month),
            selected_date: use_signal(|| (year, month, day)),

            selected_year: use_signal(|| 0),
            selected_month: use_signal(|| 0),
            selected_day: use_signal(|| 0),
        };

        Ok(ctrl)
    }
    pub fn get_days(&self) -> Vec<Vec<u32>> {
        let mut this_month_days = vec![];
        for i in 1..(self.last_date_of_month)() + 1 {
            this_month_days.push(i);
        }
        let remainder =
            6 - self.translate_num_days((self.last_weekday_of_month)().num_days_from_monday());
        let mut next_month_days = vec![];
        if remainder > 0 {
            for i in 1..remainder + 1 {
                next_month_days.push(i);
            }
        }
        let needs = self.translate_num_days((self.weekday_of_month)().num_days_from_monday());
        let mut prev_month_days = vec![];
        let last_of_prev = (self.last_date_of_prev_month)();
        if needs > 0 {
            for i in (last_of_prev - needs + 1)..last_of_prev + 1 {
                prev_month_days.push(i);
            }
        }
        vec![prev_month_days, this_month_days, next_month_days]
    }

    pub fn translate_num_days(&self, num_days: u32) -> u32 {
        if num_days == 0 {
            1
        } else if num_days == 1 {
            2
        } else if num_days == 2 {
            3
        } else if num_days == 3 {
            4
        } else if num_days == 4 {
            5
        } else if num_days == 5 {
            6
        } else {
            0
        }
    }

    pub fn handle_select(&mut self, year: i32, month: u32, day: u32) {
        let weekday_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap().weekday();
        let last_date_of_month = Self::find_last_date_of_month(year, month, None);
        let (next_year, next_month) = Self::next_month(year, month);
        let last_date_of_prev_month = Self::find_last_date_of_month(next_year, next_month, None);
        let last_weekday_of_month = NaiveDate::from_ymd_opt(year, month, last_date_of_month)
            .unwrap()
            .weekday();
        self.year.set(year);
        self.month.set(month);
        self.day.set(day);
        self.weekday_of_month.set(weekday_of_month);
        self.last_weekday_of_month.set(last_weekday_of_month);
        self.last_date_of_month.set(last_date_of_month);
        self.last_date_of_prev_month.set(last_date_of_prev_month);
        self.selected_date.set((year, month, day));
        tracing::debug!("weekday of this month: {:?}", (self.weekday_of_month)());
        tracing::debug!(
            "last weekday of this month: {:?}",
            (self.last_weekday_of_month)()
        );
    }
    pub fn find_last_date_of_month(year: i32, month: u32, day: Option<u32>) -> u32 {
        match NaiveDate::from_ymd_opt(year, month, day.unwrap_or(31)) {
            Some(date) => date.day(),
            None => Self::find_last_date_of_month(year, month, Some(day.unwrap_or(31) - 1)),
        }
    }
    pub fn previous_month(year: i32, month: u32) -> (i32, u32) {
        if month == 1 {
            (year - 1, 12)
        } else {
            (year, month - 1)
        }
    }
    pub fn next_month(year: i32, month: u32) -> (i32, u32) {
        if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        }
    }
    pub fn handle_previous_month(&mut self) {
        let (year, month) = Self::previous_month((self.year)(), (self.month)());
        self.handle_select(year, month, (self.day)());
    }
    pub fn handle_next_month(&mut self) {
        let (year, month) = Self::next_month((self.year)(), (self.month)());
        self.handle_select(year, month, (self.day)());
    }
    pub fn get_year(&self) -> i32 {
        (self.year)()
    }
    pub fn get_month(&self) -> u32 {
        (self.month)()
    }

    pub fn get_selected_year(&self) -> i32 {
        (self.selected_year)()
    }
    pub fn get_selected_month(&self) -> u32 {
        (self.selected_month)()
    }
    pub fn get_selected_day(&self) -> u32 {
        (self.selected_day)()
    }
}
