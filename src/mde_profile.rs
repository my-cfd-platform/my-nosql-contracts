service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("slippage_lader")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MdeProfile {
    pub id: String,
    pub instruments: Vec<MdeProfileInstrumentMyNoSqlModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MdeProfileInstrumentMyNoSqlModel {
    pub id: String,
    pub slippage_ladder: Option<String>,
}

impl MdeProfile {
    pub fn generate_partition_key() -> &'static str {
        "mdep"
    }

    pub fn generate_row_key(id: &str) -> String {
        id.to_string()
    }
}
