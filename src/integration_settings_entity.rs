use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("integration-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegrationSettingsMyNoSqlEntity {
    #[serde(rename = "Name")]
    name: String,
}

impl IntegrationSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "is"
    }

    pub fn generate_row_key(api_key: &str) -> &str {
        api_key
    }

    pub fn get_api_key(&self) -> &str {
        &self.row_key
    }
}
