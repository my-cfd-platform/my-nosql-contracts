use chrono::Weekday;
use serde::*;
use service_sdk::rust_extensions::date_time::{DateTimeAsMicroseconds, DateTimeStruct, TimeStruct};

pub trait TradingInstrument {
    fn get_id(&self) -> &str;

    fn get_day_offs(&self) -> &[TradingInstrumentDayOff];

    fn is_day_off(&self, now: DateTimeAsMicroseconds) -> bool {
        let dt: DateTimeStruct = now.into();

        let microseconds_with_in_week_now = dt.time.to_micro_second_withing_week(dt.dow.unwrap());

        for day_off in self.get_day_offs() {
            let time_from = TimeStruct::parse_from_str(day_off.time_from.as_str());
            if time_from.is_none() {
                continue;
            }

            let dow_from: Weekday = to_week_day(self.get_id(), day_off.dow_from);

            let micro_seconds_from = time_from.unwrap().to_micro_second_withing_week(dow_from);

            let time_to = TimeStruct::parse_from_str(day_off.time_to.as_str());
            if time_to.is_none() {
                continue;
            }

            let dow_to: Weekday = to_week_day(self.get_id(), day_off.dow_to);
            let micro_seconds_to = time_to.unwrap().to_micro_second_withing_week(dow_to);

            if micro_seconds_from <= microseconds_with_in_week_now
                && microseconds_with_in_week_now <= micro_seconds_to
            {
                return true;
            }
        }

        false
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

fn to_week_day(id: &str, value: i32) -> Weekday {
    match value {
        0 => Weekday::Mon,
        1 => Weekday::Tue,
        2 => Weekday::Wed,
        3 => Weekday::Thu,
        4 => Weekday::Fri,
        5 => Weekday::Sat,
        6 => Weekday::Sun,
        _ => panic!("Invalid week day '{}' for instrument: {}", value, id),
    }
}

#[cfg(test)]
mod tests {
    use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{TradingInstrument, TradingInstrumentDayOff};

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
                dow_from: 0,
                time_from: "12:00:00".to_string(),
                dow_to: 0,
                time_to: "12:30:00".to_string(),
            }],
        };

        //Monday but before 12:00
        let now = DateTimeAsMicroseconds::from_str("2024-07-15T00:00:00").unwrap();
        assert!(!instrument.is_day_off(now));

        //Monday but 12:00
        let now = DateTimeAsMicroseconds::from_str("2024-07-15T12:00:00").unwrap();
        assert!(instrument.is_day_off(now));

        //Monday but 12:30
        let now = DateTimeAsMicroseconds::from_str("2024-07-15T12:30:00").unwrap();
        assert!(instrument.is_day_off(now));

        //Monday but 12:30:01
        let now = DateTimeAsMicroseconds::from_str("2024-07-15T12:30:01").unwrap();
        assert!(!instrument.is_day_off(now));
    }
}
