
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use holiday_jp::HolidayService;
use chrono::{Local, Datelike, DateTime, TimeZone, Duration};

pub struct BusinessDayService;

impl BusinessDayService {

    pub fn is_businessday(&self, year: i32, month: u32, day: u32, at_businessday: u32) -> bool {
        let date = Local.ymd(year, month, day).and_hms(0, 0, 0);
        date == self.get_businessday(date.year(), date.month(), at_businessday)
    }

    pub fn get_businessday(&self, year: i32, month: u32, at_businessday: u32) -> DateTime<Local> {
        let first = Local.ymd(year, month, 1).and_hms(0, 0, 0);
        self.get_businessday_recursively(first, at_businessday, 0)
    }

    fn get_businessday_recursively(&self, date: DateTime<Local>, businessday: u32, count: u32) -> DateTime<Local> {

        if count == businessday {
            return date - Duration::days(1)
        }

        if HolidayService.is_holiday(date) {
            let next = date + Duration::days(1);
            return self.get_businessday_recursively(next, businessday, count)
        }

        let next = date + Duration::days(1);
        return self.get_businessday_recursively(next, businessday, count + 1)
    }
}
