service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("mde-profiles")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MdeProfileNoSqlEntity {
    pub id: String,
    pub instruments: Vec<InstrumentLadderModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentLadderModel {
    pub id: String,
    pub ladder_id: Option<String>,
}

impl MdeProfileNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "profile"
    }

    pub fn generate_row_key(id: &str) -> String {
        id.to_string()
    }
}
