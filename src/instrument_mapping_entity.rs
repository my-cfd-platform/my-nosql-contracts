use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();
use std::collections::HashMap;
#[my_no_sql_entity("instrument-mapping")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentMappingEntity {
    #[serde(rename = "Map")]
    pub map: HashMap<String, String>,
}

impl InstrumentMappingEntity {
    pub const PARTITION_KEY: &'static str = "im";

    pub fn get_lp_id(&self) -> &str {
        self.row_key.as_str()
    }
}
