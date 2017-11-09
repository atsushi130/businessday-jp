
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate businessday_jp;
extern crate chrono;

#[cfg(test)]
mod businessday_service_tests {

    use businessday_jp::BusinessDayService;
    use chrono::{Local, TimeZone};

    #[test]
    fn is_businessday() {
        let date   = Local.ymd(2017, 1, 5).and_hms(0, 0, 0);
        let result = BusinessDayService.is_businessday(date, 2);
        assert_eq!(true, result);
    }

    #[test]
    fn is_not_businessday() {
        let date   = Local.ymd(2017, 1, 5).and_hms(0, 0, 0);
        let result = BusinessDayService.is_businessday(date, 1);
        assert_eq!(false, result);
    }

    #[test]
    fn get_businessday() {
        let expect = Local.ymd(2017, 1, 5).and_hms(0, 0, 0);
        let result = BusinessDayService.get_businessday(2017, 1, 2);

        assert_eq!(expect, result);
    }
}