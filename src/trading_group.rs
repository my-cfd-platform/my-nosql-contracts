use serde::{Deserialize, Serialize};

// cSpell:disable
#[my_no_sql_macros::my_no_sql_entity("live-tradinggroups")]
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
