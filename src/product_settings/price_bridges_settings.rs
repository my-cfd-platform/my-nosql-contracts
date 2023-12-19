use std::collections::HashMap;

use serde::*;

//Key = bridge id (e.g. "Binance", "Yourbourse")
service_sdk::macros::use_my_no_sql_entity!();
#[enum_model(partition_key:"price-bridges", row_key: "settings")]
#[derive(Serialize, Deserialize, Clone)]
pub struct PriceBridgesSettings {
    pub bridges: HashMap<String, String>,
}
