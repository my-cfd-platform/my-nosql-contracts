service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("mde-liquidity")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MdeLiquidityMyNoSqlEntity {
    pub id: String,
    pub name: String,
    pub levels: Vec<MdeLiquidityLevel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MdeLiquidityLevel{
    pub volume: f64,
    pub volume_deviation: f64,
    pub markup: f64,
    pub markup_deviation: f64,
}