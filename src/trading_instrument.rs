use serde::*;

pub trait TradingInstrument {
    fn get_id(&self) -> &str;

    fn get_day_offs(&self) -> &[TradingInstrumentDayOff];
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TradingInstrumentDayOff {
    pub dow_from: i32,
    pub time_from: String,
    pub dow_to: i32,
    pub time_to: String,
}
