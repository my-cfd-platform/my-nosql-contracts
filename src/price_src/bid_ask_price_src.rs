use serde::{Deserialize, Serialize};
use service_sdk::rust_extensions::sorted_vec::EntityWithStrKey;
service_sdk::macros::use_my_no_sql_entity!();
#[my_no_sql_entity("bid-ask-src")]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct BidAskPriceSrc {
    pub bid: f64,
    pub ask: f64,
    pub dt: String,
    pub src_id: String,
}

impl BidAskPriceSrc {
    pub fn generate_partition_key(src_id: &str) -> &str {
        &src_id
    }

    pub fn generate_row_key(&self) -> &str {
        &self.row_key
    }
}

impl EntityWithStrKey for BidAskPriceSrc {
    fn get_key(&self) -> &str {
        &self.row_key
    }
}
