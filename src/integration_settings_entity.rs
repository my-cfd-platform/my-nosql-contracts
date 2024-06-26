use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("integration-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegrationTenantMyNoSqlEntity {
    pub name: String,
    pub api_keys: Vec<String>,
    pub allowed_ip: Option<Vec<String>>,
    pub allowed_trading_groups: Option<Vec<String>>,
    pub brand: Option<String>,
}

impl IntegrationTenantMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "it"
    }

    pub fn generate_row_key(integration_tenant_id: &str) -> &str {
        integration_tenant_id
    }

    pub fn is_my_configuration(&self, api_key_to_check: &str) -> bool {
        for api_key in self.api_keys.as_slice() {
            if api_key == api_key_to_check {
                return true;
            }
        }
        false
    }
}
