service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("slippage_lader")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SlippageLadderMyNoSqlEntity {
    pub id: String,
    pub levels: Vec<SlippageLadderLevelNoSqlModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlippageLadderLevelNoSqlModel{
    pub from_volume: f64,
    pub volume_ration: f64,
    pub markup: f64,
    pub markup_ratio: f64,

}

impl SlippageLadderMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "sl"
    }

    pub fn generate_row_key(id: &str) -> String {
        id.to_string()
    }
}
