use serde::{Deserialize, Serialize};
pub use service_sdk::my_no_sql_sdk;
use std::collections::HashMap;
#[my_no_sql_sdk::macros::my_no_sql_entity("instrument-mapping")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentMappingEntity {
    #[serde(rename = "LpId")]
    pub liquidity_provider_id: String,
    #[serde(rename = "Map")]
    pub map: HashMap<String, String>,
}
