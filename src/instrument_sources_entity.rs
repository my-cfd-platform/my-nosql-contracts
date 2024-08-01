use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("instrument-sources")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentSourcesEntity {
    #[serde(rename = "SourceId")]
    pub source_id: String,
}

impl InstrumentSourcesEntity {
    pub const PARTITION_KEY: &'static str = "qg";

    pub fn generate_row_key(instrument_id: &str) -> &str {
        instrument_id
    }

    pub fn get_instrument_id(&self) -> &str {
        &self.row_key
    }
}
