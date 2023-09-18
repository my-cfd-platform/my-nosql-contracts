use std::collections::HashMap;

use serde::{Deserialize, Serialize};
pub use service_sdk::my_no_sql_sdk;
use service_sdk::rust_extensions::StrOrString;
#[my_no_sql_sdk::macros::my_no_sql_entity("key-value-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct KeyValueCacheMyNoSqlEntity {
    pub key_value: HashMap<String, String>,
}

impl KeyValueCacheMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(trader_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_id.into()
    }
}
