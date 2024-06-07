use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();
#[my_no_sql_entity("price-sources")]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct PriceSourceIdMyNoSqlEntity {}

impl PriceSourceIdMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "s"
    }

    pub fn get_price_source_id(&self) -> &str {
        &self.row_key
    }
}
