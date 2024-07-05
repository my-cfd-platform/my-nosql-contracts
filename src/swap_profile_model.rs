use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::SwapScheduleNoSqlModel;
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
pub enum SwapScheduleType{
    Percent(SwapProfileInstrumentNoSqlModel),
    Points(SwapProfileInstrumentNoSqlModel)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwapProfileInstrumentNoSqlModel{
    pub long: f64,
    pub short: f64,
    pub schedules: Vec<SwapScheduleNoSqlModel>
}
