use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_no_sql_entity!();

// cSpell:disable
#[my_no_sql_entity("candle-min-max-keys")]
// cSpell:enable
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CandleMinMaxKey {
    pub min_key: i64,
    pub max_key: i64,
}

impl CandleMinMaxKey {
    pub fn generate_partition_key(instrument_id: &str) -> &str {
        &instrument_id
    }
    pub fn generate_row_key(candle_type_as_u8: u8) -> String {
        candle_type_as_u8.to_string()
    }
}
