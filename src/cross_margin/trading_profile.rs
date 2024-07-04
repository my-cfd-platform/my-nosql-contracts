use serde::*;
service_sdk::macros::use_my_no_sql_entity!();
#[my_no_sql_entity("tradingprofiles-cross")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingProfileNoSqlEntity {
    pub id: String,
    pub stop_out_percent: f64,
    pub is_a_book: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_call_percent: Option<f64>,
    pub instruments: Vec<TradingProfileInstrument>,
    pub leverages: Vec<f64>,
    pub collateral_currencies: Vec<String>,
    pub initial_deposit: f64,
    pub hedge_margin_coefficient: f64,
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
    pub stop_out_percent: Option<f64>,
    pub instrument_max_leverage: Option<f64>
}

impl TradingProfileNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "profile"
    }
}
