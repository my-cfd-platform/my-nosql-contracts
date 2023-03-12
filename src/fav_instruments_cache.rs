use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("fav-instruments-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FavInstrumentsCacheMyNoSqlEntity {
    pub by_account_id: HashMap<String, Vec<String>>,
}

impl FavInstrumentsCacheMyNoSqlEntity {
    pub fn generate_partition_key(trader_id: &str) -> &str {
        trader_id
    }

    pub fn generate_row_key_as_web() -> &'static str {
        "w"
    }

    pub fn generate_row_key_as_mobile() -> &'static str {
        "m"
    }
}
