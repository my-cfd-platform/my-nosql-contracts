use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();
use crate::TradingInstrumentDayOff;
#[my_no_sql_entity("instruments-cross")]
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
    pub lot_size: f64,
    pub days_off: Vec<TradingInstrumentDayOff>,
}

impl TradingInstrumentNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "i"
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}

impl crate::TradingInstrument for TradingInstrumentNoSqlEntity {
    fn get_id(&self) -> &str {
        &self.row_key
    }

    fn get_day_offs(&self) -> &[TradingInstrumentDayOff] {
        &self.days_off
    }
}
