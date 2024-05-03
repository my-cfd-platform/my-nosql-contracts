use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("key-value-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct KycProfileMyNoSqlEntity {
    pub ask_kyc_on_deposit: Option<bool>,
    pub ask_kyc_on_withdraw: Option<bool>,
    pub ask_kyc_on_trade: Option<bool>,
}

impl KycProfileMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "kyc"
    }

    pub fn generate_row_key(profile_id: &'static str) -> &'static str {
        profile_id
    }

    pub fn get_profile_id(&self) -> &str {
        &self.row_key
    }
}
