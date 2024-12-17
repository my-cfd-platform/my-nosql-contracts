service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("mde-ladders")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MdeLaddersMyNoSqlEntity {
    pub id: String,
    pub levels: Vec<MdeLadderModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MdeLadderModel{
    pub level: f64,
    pub volume_deviation: f64,
    pub markup: f64,
    pub markup_deviation: f64,

}