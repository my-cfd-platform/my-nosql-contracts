use std::collections::HashMap;

use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"price-feed", row_key: "binance")]
#[derive(Serialize, Deserialize, Clone)]
pub struct BinanceFeedSettings {
    pub instruments_mapping: HashMap<String, String>,
}
