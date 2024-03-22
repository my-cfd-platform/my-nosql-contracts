use serde::*;
service_sdk::macros::use_my_no_sql_entity!();
#[my_no_sql_entity("live-tradingprofiles")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingProfileNoSqlEntity {
    pub id: String,
    pub margin_call_percent: f64,
    pub stop_out_percent: f64,
    pub position_topping_up_percent: f64,
    pub is_a_book: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_call_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topping_up_percent: Option<f64>,
    pub instruments: Vec<TradingProfileInstrument>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingProfileInstrument {
    pub id: String,
    pub min_operation_volume: f64,
    pub max_operation_volume: f64,
    pub max_position_volume: f64,
    pub open_position_min_delay_ms: i32,
    pub open_position_max_delay_ms: i32,
    pub leverages: Vec<i32>,
    pub stop_out_percent: Option<f64>,
}

impl TradingProfileNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "profile"
    }
}
