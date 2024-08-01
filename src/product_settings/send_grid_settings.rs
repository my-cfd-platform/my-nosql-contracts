use std::collections::HashMap;

use serde::*;

use super::EmailType;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"emails", row_key: "send-grid")]
#[derive(Serialize, Deserialize, Clone)]
pub struct SendGridSettingsModel {
    pub send_grid_api_key: String,
    pub seq_conn_string: String,
    pub from_email: String,

    pub email_ids: HashMap<String, String>,
}

impl SendGridSettingsModel {
    pub fn get_email_id(&self, email_type: EmailType) -> Option<String> {
        let result = self.email_ids.get(email_type.as_str())?;
        Some(result.to_string())
    }
}
