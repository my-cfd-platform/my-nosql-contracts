use std::collections::HashMap;

use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_no_sql_entity!();
#[my_no_sql_entity("swap-profiles")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SwapProfileNoSqlModel {
    pub id: String,
    pub name: String,
    pub instruments_swaps: HashMap<String, SwapScheduleType>,
}

impl SwapProfileNoSqlModel {
    pub fn generate_partition_key(id: &'static str) -> &'static str {
        id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]

pub enum SwapScheduleType {
    Percent(SwapProfileInstrumentNoSqlModel),
    Points(SwapProfileInstrumentNoSqlModel),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SwapProfileInstrumentNoSqlModel {
    pub long: f64,
    pub short: f64,
    pub schedules: Vec<String>,
}

#[test]
pub fn test_serialize() {
    let json_data = r#"
    [
        {
            "Id": "test",
            "Name": "Test",
            "InstrumentsSwaps": {
                "AAL": {
                    "Percent": {
                        "Long": 1.0,
                        "Short": 1.0,
                        "Schedules": [
                            "test"
                        ]
                    }
                },
                "AAPL": {
                    "Percent": {
                        "Long": -1.0,
                        "Short": 1.0,
                        "Schedules": [
                            "test"
                        ]
                    }
                },
                "AMD": {
                    "Percent": null,
                    "Points": {
                        "Long": 0.0,
                        "Short": 0.0,
                        "Schedules": [
                            "test"
                        ]
                    }
                }
            },
            "PartitionKey": "swap_profile",
            "RowKey": "test",
            "TimeStamp": "2024-07-18T21:44:44.8151"
        }
    ]
    "#;

    let model: Vec<SwapProfileNoSqlModel> = serde_json::from_str(&json_data).unwrap();

    println!("{:#?}", model);
}

#[test]
pub fn test_serialize_2() {
    let model = SwapProfileNoSqlModel {
        partition_key: "swap_profile".to_string(),
        row_key: "test".to_string(),
        time_stamp: "2024-07-18T21:44:44.8151".to_string(),
        id: "test".to_string(),
        name: "Test".to_string(),
        instruments_swaps: HashMap::from_iter(vec![
            (
                "AAL".to_string(),
                SwapScheduleType::Percent(SwapProfileInstrumentNoSqlModel {
                    long: 1.0,
                    short: 1.0,
                    schedules: vec!["test".to_string()],
                }),
            ),
            (
                "AAPL".to_string(),
                SwapScheduleType::Percent(SwapProfileInstrumentNoSqlModel {
                    long: -1.0,
                    short: 1.0,
                    schedules: vec!["test".to_string()],
                }),
            ),
            (
                "AMD".to_string(),
                SwapScheduleType::Percent(SwapProfileInstrumentNoSqlModel {
                    long: 0.0,
                    short: 0.0,
                    schedules: vec!["test".to_string()],
                }),
            ),
            (
                "BTCYFT".to_string(),
                SwapScheduleType::Points(SwapProfileInstrumentNoSqlModel {
                    long: 0.0,
                    short: 0.0,
                    schedules: vec!["test2".to_string(), "test3".to_string()],
                }),
            ),
        ]),
    };

    println!("{}", serde_json::to_string(&model).unwrap());
}
