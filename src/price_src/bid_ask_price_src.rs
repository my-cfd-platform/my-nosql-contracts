use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();
#[my_no_sql_entity("bid-ask-src")]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct PriceSourceIdMyNoSqlEntity {
    pub bid: f64,
    pub ask: f64,
    pub dt: String,
    pub src_id: String,
}

impl PriceSourceIdMyNoSqlEntity {
    pub fn generate_partition_key(src_id: &str) -> &str {
        &src_id
    }

    pub fn generate_row_key(&self) -> &str {
        &self.row_key
    }
}
