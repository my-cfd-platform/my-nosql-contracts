use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("instruments")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingInstrumentNoSqlEntity {
    pub name: String,
    pub digits: i32,
    pub base: String,
    pub quote: String,
    pub tick_size: f64,
    pub swap_schedule_id: Option<String>,
    pub group_id: Option<String>,
    pub weight: Option<i32>,
    pub day_timeout: Option<i32>,
    pub night_timeout: Option<i32>,
    pub trading_disabled: bool,
    pub days_off: Vec<TradingInstrumentDayOff>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingInstrumentDayOff {
    pub dow_from: i32,
    pub time_from: String,
    pub dow_to: i32,
    pub time_to: String,
}

impl TradingInstrumentNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "i"
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}