use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("instrumentsgroups")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingInstrumentGroupNoSqlEntity {
    pub id: String,
    pub name: String,
    pub weight: i32,
}

impl TradingInstrumentNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "ig"
    }
}