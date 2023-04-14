use serde::*;

const DAILY_PK: &str = "Daily";
const HOURLY_PK: &str = "Hourly";
const MINUTE_PK: &str = "Minute";

#[my_no_sql_macros::my_no_sql_entity("price-change-snapshots")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PriceChangeSnapshotNoSqlEntity {
    pub previous_price: f64,
    pub current_price: f64,
    pub change_percent: Option<f64>,
    pub name: String,
    pub period: String
}

impl<'s> PriceChangeSnapshotNoSqlEntity {
    pub fn get_daily_pk() -> &'s str{
        DAILY_PK
    }

    pub fn get_hourly_pk() -> &'s str{
        HOURLY_PK
    }
    
    pub fn get_minute_pk() -> &'s str{
        MINUTE_PK
    }
}
