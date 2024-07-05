use std::collections::HashMap;

use chrono::Weekday;
use serde::{Serialize, Deserialize};
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("swap-schedules")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SwapScheduleNoSqlModel {
    pub id: String,
    pub name: String,
    //key - dow, value - day schedule
    pub weak_schedules: HashMap<Weekday, SwapDaySchedule>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwapDaySchedule{
    pub time: String,
    pub swap_multiplier: f64
}