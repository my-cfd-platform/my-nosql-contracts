use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();
#[my_no_sql_entity("swap-profile")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SwapProfileNoSqlModel {
    pub id: String,
    pub instrument_id: String,
    pub long: f64,
    pub short: f64,
}

impl SwapProfileNoSqlModel {
    pub fn generate_partition_key(group_id: &'static str) -> &'static str {
        group_id
    }
}
