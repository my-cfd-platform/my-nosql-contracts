use serde::{Serialize, Deserialize};
service_sdk::macros::use_my_no_sql_entity!();
#[my_no_sql_entity("instrumentsgroups")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingInstrumentGroupNoSqlEntity {
    pub id: String,
    pub name: String,
    pub weight: i32,
    pub hidden: bool,
}
