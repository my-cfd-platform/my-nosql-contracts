

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[my_no_sql_macros::my_no_sql_entity("instrument-mapping")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentMappingEntity {
    #[serde(rename = "LpId")]
    pub liquidity_provider_id: String,
    #[serde(rename = "Map")]
    pub map: HashMap<String, String>,
}