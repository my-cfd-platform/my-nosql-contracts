use serde::{Serialize, Deserialize};
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("swap-schedule")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SwapScheduleNoSqlModel {
    pub id: String,
    pub date_of_week: i32,
    pub time: String,
    pub amount: i32,
}
