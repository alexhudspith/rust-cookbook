#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::thread;
    use std::thread::sleep;
    use std::time::{Duration, Instant, SystemTime};

    use chrono::{Datelike, DateTime, Days, Local, LocalResult, Months, NaiveDate, NaiveDateTime, NaiveTime, Timelike, TimeZone, Utc, Weekday};
    // Needed for Offset::fix()
    use chrono::Offset;
    use chrono_tz::{America, Asia, Europe, Tz};

    const ISO_8601: &str = "%Y-%m-%dT%H:%M:%S";
    const ISO_8601_TZ: &str = "%Y-%m-%dT%H:%M:%S%z";
    const ZERO: Duration = Duration::from_nanos(0);

    #[test]
    fn instant() {
        let t0 = Instant::now();
        println!("{:?}", t0);

        let sleep = Duration::from_millis(100);
        thread::sleep(sleep);

        let t1 = Instant::now();
        println!("{:?}", t1);

        assert!(t1 >= t0 + sleep);

        let d: Duration = t1 - t0;
        assert!(d >= sleep);
        assert!(t1.checked_duration_since(t0) > Some(sleep));
        assert_eq!(t0 + d, t1);

        assert_eq!(t0 - t1, ZERO);
        assert_eq!(t0.duration_since(t1), ZERO);
        assert_eq!(t0.saturating_duration_since(t1), ZERO);
        assert!(t0.checked_duration_since(t1).is_none());

        assert!(t0 + Duration::from_nanos(1) > t0);
        assert!(t0 - Duration::from_nanos(1) < t0);
    }

    #[test]
    fn system() {
        let t0 = SystemTime::now();
        println!("{:?}", t0);

        let sleep = Duration::from_millis(100);
        thread::sleep(sleep);

        let t1 = SystemTime::now();
        println!("{:?}", t1);

        assert!(t1 >= t0 + sleep);

        // let d: Duration = t1 - t0;
        // assert!(d >= sleep);
        // assert!(t1.checked_duration_since(t0) > Some(sleep));
        // assert_eq!(t0 + d, t1);

        // assert_eq!(t0 - t1, ZERO);
        assert!(t0.duration_since(t1).is_err());

        assert!(t0 + Duration::from_nanos(1) > t0);
        assert!(t0 - Duration::from_nanos(1) < t0);

        let dt: DateTime<Tz> = NaiveDateTime::parse_from_str("2023-03-26T00:59:59", ISO_8601)
            .unwrap()
            .and_local_timezone(Europe::London)
            .unwrap();
        println!("{:?}", dt);
        println!("{:?}", dt + Duration::from_secs(1));
    }

    #[test]
    fn chrono() {
        let utc: DateTime<Utc> = Utc::now();
        println!("{}", utc);
        println!("{:?}", utc);
        println!();
        sleep(Duration::from_millis(10));

        let local: DateTime<Local> = Local::now();
        println!("{}", local);
        println!("{:?}", local);
        println!();
        sleep(Duration::from_millis(10));

        let tz: DateTime<Tz> = America::New_York.from_utc_datetime(&local.naive_utc());
        println!("{}", tz);
        println!("{:?}", tz);
        println!();

        let system_time: SystemTime = utc.into();
        let dt: DateTime<Utc> = system_time.into();
        assert_eq!(dt, utc);
        let system_time2: SystemTime = dt.into();
        assert_eq!(system_time2, system_time);

        let system_time: SystemTime = local.into();
        let dt: DateTime<Local> = system_time.into();
        assert_eq!(dt, local);
        let system_time2: SystemTime = dt.into();
        assert_eq!(system_time2, system_time);

        let system_time: SystemTime = tz.into();
        let dt: DateTime<Utc> = system_time.into();
        assert_eq!(dt, tz);
        let system_time2: SystemTime = dt.into();
        assert_eq!(system_time2, system_time);
    }

    #[test]
    fn chrono_eq() {
        let dt_naive_21 = NaiveDateTime::parse_from_str("2023-10-21T22:29:33", ISO_8601)
            .unwrap();
        let dt_utc_21 = dt_naive_21.and_utc();
        let dt_naive_22 = NaiveDateTime::parse_from_str("2023-10-22T07:29:33", ISO_8601)
            .unwrap();
        let dt_tok_22 = dt_naive_22
            .and_local_timezone(Asia::Tokyo)
            .unwrap();
        let dt_tok_21 = dt_naive_21
            .and_local_timezone(Asia::Tokyo)
            .unwrap();

        // Test setup: must be same instant
        let sys_utc_21: SystemTime = dt_utc_21.into();
        let sys_tok_22: SystemTime = dt_tok_22.into();
        assert_eq!(sys_utc_21, sys_tok_22, "Test setup failure");

        // Same instant
        assert_eq!(dt_utc_21, dt_tok_22);
        // Different instant (same local time)
        assert_ne!(dt_utc_21, dt_tok_21);

        // Same naive time
        assert_eq!(dt_utc_21.naive_utc(), dt_naive_21);
        // Same naive time viewed in UTC
        assert_eq!(dt_utc_21.naive_local(), dt_utc_21.naive_local());
        // Same naive time viewed in Tokyo tz
        assert_eq!(dt_tok_22.naive_local(), dt_naive_22);
        // Same naive time viewed in UTC
        assert_eq!(dt_tok_22.naive_utc(), dt_utc_21.naive_utc());
    }

    #[test]
    fn chrono_daylight_saving_forward() {
        // One second before GMT -> BST switch (clock jumps forward 1 hour, 01:mm:ss doesn't exist)
        let dt0: DateTime<Tz> = NaiveDateTime::parse_from_str("2023-03-26T00:59:59", ISO_8601)
            .unwrap()
            .and_local_timezone(Europe::London)
            .unwrap();

        assert_eq!(dt0.year(), 2023);
        assert_eq!(dt0.month(), 3);
        assert_eq!(dt0.day(), 26);
        assert_eq!(dt0.hour(), 0);
        assert_eq!(dt0.minute(), 59);
        assert_eq!(dt0.second(), 59);
        assert_eq!(dt0.timezone(), Europe::London);
        assert_eq!(dt0.offset().fix().local_minus_utc(), 0);

        let dt1 = dt0 + Duration::from_secs(1);
        assert_eq!(dt1.date_naive(), dt0.date_naive());
        assert_eq!(dt1.time(), "02:00:00".parse::<NaiveTime>().unwrap());
        assert_eq!(dt1.timezone(), Europe::London);
        assert_eq!(dt1.offset().fix().local_minus_utc(), 3600);

        let local_result = NaiveDateTime::parse_from_str("2023-03-26T01:00:00", ISO_8601)
            .unwrap()
            .and_local_timezone(Europe::London);
        assert!(matches!(local_result, LocalResult::None));
    }

    #[test]
    fn chrono_daylight_saving_backward() {
        // One second before BST -> GMT switch (clock jumps backward 1 hour, 01:mm:ss exists twice)
        let dt0: DateTime<Tz> = NaiveDateTime::parse_from_str("2023-10-29T00:59:59", ISO_8601)
            .unwrap()
            .and_utc()
            .with_timezone(&Europe::London);

        assert_eq!(dt0.year(), 2023);
        assert_eq!(dt0.month(), 10);
        assert_eq!(dt0.day(), 29);
        assert_eq!(dt0.hour(), 1);
        assert_eq!(dt0.minute(), 59);
        assert_eq!(dt0.second(), 59);
        assert_eq!(dt0.timezone(), Europe::London);
        assert_eq!(dt0.offset().fix().local_minus_utc(), 3600);

        let dt1 = dt0 + Duration::from_secs(1);
        assert_eq!(dt1.date_naive(), dt0.date_naive());
        assert_eq!(dt1.time(), "01:00:00".parse().unwrap());
        assert_eq!(dt1.timezone(), Europe::London);
        assert_eq!(dt1.offset().fix().local_minus_utc(), 0);

        // Ambiguous time
        let local_result = NaiveDateTime::parse_from_str("2023-10-29T01:00:00", ISO_8601)
            .unwrap()
            .and_local_timezone(Europe::London);

        // BST possibility
        let min = DateTime::parse_from_str("2023-10-29T01:00:00+01:00", ISO_8601_TZ)
            .unwrap()
            .with_timezone(&Europe::London);
        // GMT possibility
        let max = DateTime::parse_from_str("2023-10-29T01:00:00+00:00", ISO_8601_TZ)
            .unwrap()
            .with_timezone(&Europe::London);

        assert!(matches!(local_result, LocalResult::Ambiguous(a, b) if (a, b) == (min, max)));
    }

    #[test]
    fn skip_months() {
        let d0 = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
        let d1 = d0 + Months::new(1);
        assert_eq!(d1, NaiveDate::from_ymd_opt(2023, 2, 28).unwrap());

        let d1 = d1 + Months::new(1);
        assert_eq!(d1, NaiveDate::from_ymd_opt(2023, 3, 28).unwrap());

        let d1 = d0 + Months::new(2);
        assert_eq!(d1, NaiveDate::from_ymd_opt(2023, 3, 31).unwrap());
    }

    #[test]
    fn skip_months_leap() {
        let d0 = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap();
        assert!(d0.leap_year());

        let d1 = d0 + Months::new(12);
        assert_eq!(d1, NaiveDate::from_ymd_opt(2021, 2, 28).unwrap());

        let d1 = d0 + Months::new(4 * 12);
        assert_eq!(d1, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
    }

    struct BusinessCalendar {}
    impl BusinessCalendar {
        const ITER_LIMIT: usize = 60;

        fn is_business_day(d: NaiveDate) -> bool {
            // Hypothetical
            let day = d.borrow().weekday();
            day != Weekday::Sat && day != Weekday::Sun
        }

        fn following(d: NaiveDate) -> NaiveDate {
            d.iter_days()
                .take(Self::ITER_LIMIT)
                .find(|&r| Self::is_business_day(r))
                .unwrap_or_else(||
                    panic!("No business days within {} days of {}", Self::ITER_LIMIT, d)
                )
        }

        fn modified_following(d: NaiveDate) -> Option<NaiveDate> {
            if let Some(result) = d.iter_days()
                .take_while(|&r| r.month() == d.month())
                .find(|&r| Self::is_business_day(r)) {
                return Some(result)
            }

            d.iter_days()
                .rev()
                .skip(1)
                .take_while(|&r| r.month() == d.month())
                .find(|&r| Self::is_business_day(r))
        }
    }

    #[test]
    fn following() {
        let thu = "2023-09-28".parse().unwrap();
        let fri = "2023-09-29".parse().unwrap();
        let sat = "2023-09-30".parse().unwrap();
        let sun = "2023-10-01".parse().unwrap();
        let mon = "2023-10-02".parse().unwrap();
        assert_eq!(BusinessCalendar::following(thu), thu);
        assert_eq!(BusinessCalendar::following(fri), fri);
        assert_eq!(BusinessCalendar::following(sat), mon);
        assert_eq!(BusinessCalendar::following(sun), mon);
        assert_eq!(BusinessCalendar::following(mon), mon);
    }

    #[test]
    fn modified_following() {
        let thu = "2023-09-28".parse().unwrap();
        let fri = "2023-09-29".parse().unwrap();
        let sat = "2023-09-30".parse().unwrap();
        let sun = "2023-10-01".parse().unwrap();
        let mon = "2023-10-02".parse().unwrap();
        assert_eq!(BusinessCalendar::modified_following(thu), Some(thu));
        assert_eq!(BusinessCalendar::modified_following(fri), Some(fri));
        assert_eq!(BusinessCalendar::modified_following(sat), Some(fri));
        assert_eq!(BusinessCalendar::modified_following(sun), Some(mon));
        assert_eq!(BusinessCalendar::modified_following(mon), Some(mon));
    }

    #[test]
    fn end_of_month() {
        fn eom(d: NaiveDate) -> NaiveDate {
            d.with_day(1).unwrap() + Months::new(1) - Days::new(1)
        }

        assert_eq!("2023-09-30".parse().map(eom), "2023-09-30".parse());
        assert_eq!("2023-10-02".parse().map(eom), "2023-10-31".parse());
        assert_eq!("2020-02-15".parse().map(eom), "2020-02-29".parse());
    }
}
