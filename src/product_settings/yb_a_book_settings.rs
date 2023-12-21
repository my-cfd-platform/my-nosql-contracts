use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"a-book", row_key: "yb")]
#[derive(Serialize, Deserialize, Clone)]
pub struct YbABookSettings {
    pub sender: String,
    pub target: String,
    pub password: String,
    pub url: String,
}
