service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("mde-profiles")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MdeProfileMyNoSqlEntity {
    pub id: String,
    pub name: String,
    pub instruments: Vec<InstrumentMdeLiquidityProfile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct InstrumentMdeLiquidityProfile {
    pub id: String,
    pub profile_id: Option<String>,
}

impl MdeProfileMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "profile"
    }

    pub fn generate_row_key(id: &str) -> String {
        id.to_string()
    }
}
