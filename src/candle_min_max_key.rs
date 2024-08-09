use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_no_sql_entity!();

// cSpell:disable
#[my_no_sql_entity("candle-min-max-keys")]
// cSpell:enable
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CandleMinMaxKeyNoSqlEntity {
    pub min_key: i64,
    pub max_key: i64,
}

impl CandleMinMaxKeyNoSqlEntity {
    pub fn generate_partition_key(instrument_id: &str) -> &str {
        &instrument_id
    }
    pub fn generate_row_key(candle_type_as_u8: u8, is_bid: bool) -> String {
        if is_bid {
            return format!("bid-{}", candle_type_as_u8);
        }

        format!("ask-{}", candle_type_as_u8)
    }
}
