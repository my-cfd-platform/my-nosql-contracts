use std::collections::HashMap;

use rust_extensions::{IntoStringOrStr, StrOrString};
use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("key-value-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct KeyValueCacheMyNoSqlEntity {
    pub key_value: HashMap<String, String>,
}

impl KeyValueCacheMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(trader_id: impl IntoStringOrStr<'s>) -> StrOrString<'s> {
        trader_id.into_string_or_str()
    }
}
