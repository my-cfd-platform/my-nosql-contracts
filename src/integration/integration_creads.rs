use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
use service_sdk::my_no_sql_sdk;
#[my_no_sql_entity("integration-creads")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntegrationCreadsNoSqlEntity {
    pub id: String,
    pub name: String,
    pub secret_key: String,
}

impl IntegrationCreadsNoSqlEntity{
    pub fn generate_partition_key() -> &'static str {
        "ic"
    }
}
