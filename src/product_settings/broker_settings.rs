use serde::*;

service_sdk::macros::use_my_no_sql_entity!();
#[enum_model(partition_key:"settings", row_key: "broker")]
#[derive(Serialize, Deserialize, Clone)]
pub struct BrokerSettings {
    #[serde(rename = "personalAreaUrl")]
    pub personal_area_url:  Optional<String>,
}
