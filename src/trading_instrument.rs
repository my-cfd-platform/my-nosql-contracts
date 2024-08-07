use chrono::Weekday;
use serde::*;
use service_sdk::rust_extensions::date_time::{DateTimeAsMicroseconds, DateTimeStruct, TimeStruct};

pub trait TradingInstrument {
    fn get_id(&self) -> &str;

    fn get_day_offs(&self) -> &[TradingInstrumentDayOff];

    fn is_day_off(&self, moment: DateTimeAsMicroseconds) -> Option<&TradingInstrumentDayOff> {
        let dt: DateTimeStruct = moment.into();

        let microseconds_with_in_week_moment =
            dt.time.to_micro_second_withing_week(dt.dow.unwrap());

        for day_off in self.get_day_offs() {
            if day_off.inside_this_interval(self.get_id(), microseconds_with_in_week_moment) {
                return Some(day_off);
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingInstrumentDayOff {
    pub dow_from: i32,
    pub time_from: String,
    pub dow_to: i32,
    pub time_to: String,
}

impl TradingInstrumentDayOff {
    pub fn inside_this_interval_from_date_time(
        &self,
        id: &str,
        moment: DateTimeAsMicroseconds,
    ) -> bool {
        let dt: DateTimeStruct = moment.into();

        let microseconds_with_in_week_moment =
            dt.time.to_micro_second_withing_week(dt.dow.unwrap());

        self.inside_this_interval(id, microseconds_with_in_week_moment)
    }
    pub fn inside_this_interval(&self, id: &str, microseconds_with_in_week_moment: u64) -> bool {
        let time_from = TimeStruct::parse_from_str(self.time_from.as_str());
        if time_from.is_none() {
            return false;
        }

        let dow_from: Weekday = to_week_day(id, self.dow_from);

        let micro_seconds_from = time_from.unwrap().to_micro_second_withing_week(dow_from);

        let time_to = TimeStruct::parse_from_str(self.time_to.as_str());
        if time_to.is_none() {
            return false;
        }

        let dow_to: Weekday = to_week_day(id, self.dow_to);
        let micro_seconds_to = time_to.unwrap().to_micro_second_withing_week(dow_to);

        if micro_seconds_from < micro_seconds_to {
            if micro_seconds_from <= microseconds_with_in_week_moment
                && microseconds_with_in_week_moment <= micro_seconds_to
            {
                return true;
            }
        } else if micro_seconds_from <= microseconds_with_in_week_moment
            || microseconds_with_in_week_moment <= micro_seconds_to
        {
            return true;
        }

        false
    }
}

fn to_week_day(id: &str, value: i32) -> Weekday {
    match value {
        0 => Weekday::Sun,
        1 => Weekday::Mon,
        2 => Weekday::Tue,
        3 => Weekday::Wed,
        4 => Weekday::Thu,
        5 => Weekday::Fri,
        6 => Weekday::Sat,

        _ => panic!("Invalid week day '{}' for instrument: {}", value, id),
    }
}

#[cfg(test)]
mod tests {
    use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{TradingInstrument, TradingInstrumentDayOff};

    fn week_day_to_i32(weekday: chrono::Weekday) -> i32 {
        match weekday {
            chrono::Weekday::Sun => 0,
            chrono::Weekday::Mon => 1,
            chrono::Weekday::Tue => 2,
            chrono::Weekday::Wed => 3,
            chrono::Weekday::Thu => 4,
            chrono::Weekday::Fri => 5,
            chrono::Weekday::Sat => 6,
        }
    }

    pub struct TestTradingInstrument {
        pub id: String,
        pub day_offs: Vec<TradingInstrumentDayOff>,
    }

    impl TradingInstrument for TestTradingInstrument {
        fn get_id(&self) -> &str {
            self.id.as_str()
        }

        fn get_day_offs(&self) -> &[TradingInstrumentDayOff] {
            self.day_offs.as_slice()
        }
    }

    #[test]
    fn test_simple_cases() {
        let instrument = TestTradingInstrument {
            id: "EURUSD".to_string(),
            day_offs: vec![TradingInstrumentDayOff {
                dow_from: week_day_to_i32(chrono::Weekday::Mon),
                time_from: "12:00:00".to_string(),
                dow_to: week_day_to_i32(chrono::Weekday::Mon),
                time_to: "12:30:00".to_string(),
            }],
        };

        //Monday but before 12:00
        let now = DateTimeAsMicroseconds::from_str("2024-07-15T00:00:00").unwrap();
        assert!(instrument.is_day_off(now).is_none());

        //Monday but 12:00
        let now = DateTimeAsMicroseconds::from_str("2024-07-15T12:00:00").unwrap();
        assert!(instrument.is_day_off(now).is_some());

        //Monday but 12:30
        let now = DateTimeAsMicroseconds::from_str("2024-07-15T12:30:00").unwrap();
        assert!(instrument.is_day_off(now).is_some());

        //Monday but 12:30:01
        let now = DateTimeAsMicroseconds::from_str("2024-07-15T12:30:01").unwrap();
        assert!(instrument.is_day_off(now).is_none());
    }

    #[test]
    fn test_with_week_crossed() {
        let instrument = TestTradingInstrument {
            id: "EURUSD".to_string(),
            day_offs: vec![TradingInstrumentDayOff {
                dow_from: week_day_to_i32(chrono::Weekday::Sun),
                time_from: "23:59:59".to_string(),
                dow_to: week_day_to_i32(chrono::Weekday::Mon),
                time_to: "00:00:00".to_string(),
            }],
        };

        //Sunday
        let now = DateTimeAsMicroseconds::from_str("2024-07-14T12:30:00").unwrap();
        assert!(instrument.is_day_off(now).is_none());
        //Sunday
        let now = DateTimeAsMicroseconds::from_str("2024-07-14T23:59:59").unwrap();
        assert!(instrument.is_day_off(now).is_some());

        //Monday
        let now = DateTimeAsMicroseconds::from_str("2024-07-15T00:00:00").unwrap();
        assert!(instrument.is_day_off(now).is_some());

        //Monday
        let now = DateTimeAsMicroseconds::from_str("2024-07-15T00:00:01").unwrap();
        assert!(instrument.is_day_off(now).is_none());
    }

    #[test]
    fn test_weekend_of_audcad() {
        let instrument = TestTradingInstrument {
            id: "EURUSD".to_string(),
            day_offs: vec![TradingInstrumentDayOff {
                dow_from: week_day_to_i32(chrono::Weekday::Fri),
                time_from: "20:59:59".to_string(),
                dow_to: week_day_to_i32(chrono::Weekday::Sun),
                time_to: "21:00:00".to_string(),
            }],
        };

        //Sunday
        let now = DateTimeAsMicroseconds::from_str("2024-07-14T12:30:00").unwrap();
        assert!(instrument.is_day_off(now).is_some());

        let now = DateTimeAsMicroseconds::from_str("2024-08-02T20:59:58").unwrap();
        assert!(instrument.is_day_off(now).is_none());

        let now = DateTimeAsMicroseconds::from_str("2024-08-02T20:59:59").unwrap();
        assert!(instrument.is_day_off(now).is_some());

        //Saturday
        let now = DateTimeAsMicroseconds::from_str("2024-08-03T20:00:00").unwrap();
        assert!(instrument.is_day_off(now).is_some());

        //Sunday
        let now = DateTimeAsMicroseconds::from_str("2024-08-04T20:59:59").unwrap();
        assert!(instrument.is_day_off(now).is_some());

        //Sunday
        let now = DateTimeAsMicroseconds::from_str("2024-08-04T21:00:00").unwrap();
        assert!(instrument.is_day_off(now).is_some());

        let now = DateTimeAsMicroseconds::from_str("2024-08-01T21:00:01").unwrap();
        assert!(instrument.is_day_off(now).is_none());
    }

    #[test]
    fn test_week_day_of_xau_usd() {
        let instrument = TestTradingInstrument {
            id: "XAUUSD".to_string(),
            day_offs: vec![
                TradingInstrumentDayOff {
                    dow_from: 2,
                    time_from: "00:00:00".to_string(),
                    dow_to: 2,
                    time_to: "01:00:00".to_string(),
                },
                TradingInstrumentDayOff {
                    dow_from: 3,
                    time_from: "00:00:00".to_string(),
                    dow_to: 3,
                    time_to: "01:00:00".to_string(),
                },
                TradingInstrumentDayOff {
                    dow_from: 4,
                    time_from: "00:00:00".to_string(),
                    dow_to: 4,
                    time_to: "01:00:00".to_string(),
                },
                TradingInstrumentDayOff {
                    dow_from: 5,
                    time_from: "00:00:00".to_string(),
                    dow_to: 5,
                    time_to: "01:00:00".to_string(),
                },
                TradingInstrumentDayOff {
                    dow_from: 5,
                    time_from: "23:59:00".to_string(),
                    dow_to: 1,
                    time_to: "01:00:00".to_string(),
                },
            ],
        };

        // Wednesday
        let now = DateTimeAsMicroseconds::from_str("2024-07-31T10:12:01").unwrap();
        assert!(instrument.is_day_off(now).is_none());

        // Sunday
        let now = DateTimeAsMicroseconds::from_str("2024-08-04T10:12:01").unwrap();
        assert!(instrument.is_day_off(now).is_some());
    }
}
