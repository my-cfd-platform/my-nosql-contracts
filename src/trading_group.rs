use serde::{Deserialize, Serialize};
pub use service_sdk::my_no_sql_sdk;
// cSpell:disable
#[my_no_sql_sdk::macros::my_no_sql_entity("live-tradinggroups")]
// cSpell:enable
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingGroupNoSqlEntity {
    pub id: String,
    pub name: String,
    pub trading_profile_id: String,
    pub markup_profile_id: String,
    pub swap_profile_id: String,
    pub trading_disabled: bool,
}

impl TradingGroupNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "group"
    }
}
