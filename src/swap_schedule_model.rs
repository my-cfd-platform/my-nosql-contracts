use std::collections::HashMap;

use chrono::Weekday;
use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("swap-schedules")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SwapScheduleNoSqlModel {
    pub id: String,
    pub name: String,
    //key - dow, value - day schedule
    pub week_schedules: HashMap<Weekday, SwapDaySchedule>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SwapDaySchedule {
    pub time: String,
    pub swap_multiplier: f64,
}

#[test]
pub fn test_serialize() {
    let json_data = r#"
    [
        {
            "Id": "test",
            "Name": "FX 2PM 3x Wed",
            "WeekSchedules": {
                "Tue": {
                    "Time": "03:00:00",
                    "SwapMultiplier": 1
                },
                "Wed": {
                    "Time": "03:00:00",
                    "SwapMultiplier": 1
                },
                "Thu": {
                    "Time": "03:00:00",
                    "SwapMultiplier": 3
                },
                "Fri": {
                    "Time": "03:00:00",
                    "SwapMultiplier": 1
                },
                "Sat": {
                    "Time": "03:00:00",
                    "SwapMultiplier": 1
                }
            },
            "PartitionKey": "swap_schedule",
            "RowKey": "test",
            "TimeStamp": "2024-07-18T21:22:14.9189"
        },
        {
            "Id": "test2",
            "Name": "Crypto 2AM Daily",
            "WeekSchedules": {
                "Mon": {
                    "Time": "02:00:00",
                    "SwapMultiplier": 1
                },
                "Tue": {
                    "Time": "02:00:00",
                    "SwapMultiplier": 1
                },
                "Wed": {
                    "Time": "02:00:00",
                    "SwapMultiplier": 1
                },
                "Thu": {
                    "Time": "02:00:00",
                    "SwapMultiplier": 1
                },
                "Fri": {
                    "Time": "02:00:00",
                    "SwapMultiplier": 1
                },
                "Sat": {
                    "Time": "02:00:00",
                    "SwapMultiplier": 1
                },
                "Sun": {
                    "Time": "02:00:00",
                    "SwapMultiplier": 1
                }
            },
            "PartitionKey": "swap_schedule",
            "RowKey": "test2",
            "TimeStamp": "2024-07-18T21:41:43.07600"
        },
        {
            "Id": "test3",
            "Name": "Crypto 2PM Daily",
            "WeekSchedules": {
                "Mon": {
                    "Time": "00:00:00",
                    "SwapMultiplier": 1
                },
                "Tue": {
                    "Time": "00:00:00",
                    "SwapMultiplier": 1
                },
                "Wed": {
                    "Time": "00:00:00",
                    "SwapMultiplier": 1
                },
                "Thu": {
                    "Time": "00:00:00",
                    "SwapMultiplier": 1
                },
                "Fri": {
                    "Time": "00:00:00",
                    "SwapMultiplier": 1
                },
                "Sat": {
                    "Time": "00:00:00",
                    "SwapMultiplier": 1
                },
                "Sun": {
                    "Time": "00:00:00",
                    "SwapMultiplier": 1
                }
            },
            "PartitionKey": "swap_schedule",
            "RowKey": "test3",
            "TimeStamp": "2024-07-18T21:43:38.4940"
        }
    ]
    "#;

    let model: Vec<SwapScheduleNoSqlModel> = serde_json::from_str(&json_data).unwrap();

    println!("{:#?}", model);
}
