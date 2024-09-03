use std::collections::HashMap;

use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("price-feeds")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceFeedEntity {
    pub name: String,
    pub mapping: HashMap<String, String>,
    pub port: u32
}

impl PriceFeedEntity {
    pub const PARTITION_KEY: &'static str = "pf";

    pub fn generate_row_key(id: &str) -> &str {
        id
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
