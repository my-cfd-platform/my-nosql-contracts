use std::collections::HashMap;

use chrono::Weekday;
use serde::{Deserialize, Serialize};

use crate::{SwapDaySchedule, SwapScheduleNoSqlModel};
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

// #[test]
// pub fn test_serialize() {
//     let swap = SwapScheduleNoSqlModel {
//         partition_key: "ads".to_string(),
//         row_key: "asd".to_string(),
//         time_stamp: "asddsa".to_string(),
//         id: "asd".to_string(),
//         name: "asd".to_string(),
//         weak_schedules: HashMap::from_iter(vec![(Weekday::Mon ,SwapDaySchedule {
//             time: "as".to_string(),
//             swap_multiplier: 1233.0,
//         })]),
//     };

//     let mut hs = HashMap::new();

//     hs.insert(
//         "BTCUSD".to_string(),
//         SwapScheduleType::Percent(SwapProfileInstrumentNoSqlModel {
//             long: 254.0,
//             short: 254.0,
//             schedules: vec![swap.clone()],
//         }),
//     );

//     hs.insert(
//         "ETHUSD".to_string(),
//         SwapScheduleType::Points(SwapProfileInstrumentNoSqlModel {
//             long: 254.0,
//             short: 254.0,
//             schedules: vec![swap],
//         }),
//     );

//     let model = SwapProfileNoSqlModel {
//         partition_key: "as".to_string(),
//         row_key: "as".to_string(),
//         time_stamp: "as".to_string(),
//         id: "as".to_string(),
//         name: "as".to_string(),
//         instruments_swaps: hs,
//     };

//     println!("{}", serde_json::to_string(&model).unwrap());
// }
