service_sdk::macros::use_my_no_sql_entity!();
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[my_no_sql_entity("markup-profiles")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MarkupProfileNoSqlEntity {
    pub name: String,
    pub disabled: bool,
    pub instruments: HashMap<String, MarkupInstrumentEntity>,
}

impl MarkupProfileNoSqlEntity {
    pub const GLOBAL_PROFILE_ID: &'static str = "GLOBAL";

    pub fn generate_partition_key() -> &'static str {
        "markup"
    }

    pub fn generate_row_key<'s>(profile_id: &str) -> &str {
        profile_id
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MarkupInstrumentEntity {
    pub markup_bid: i32,
    pub markup_ask: i32,
}
