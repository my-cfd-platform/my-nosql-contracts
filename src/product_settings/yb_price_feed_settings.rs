use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"price-feed", row_key: "yb")]
#[derive(Serialize, Deserialize, Clone)]
pub struct YbPriceFeedSettings {
    pub url: String,
    pub pass: String,
    pub sender_company_id: String,
    pub target_company_id: String,
}
