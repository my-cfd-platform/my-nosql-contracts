use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_of_my_no_sql_entity(table_name:"product-settings", generate_unwraps)]
pub enum ProductSettings {
    SendGrid(SendGridSettingsModel),
}

#[enum_model(partition_key:"emails", row_key: "send-grid")]
#[derive(Serialize, Deserialize, Clone)]
pub struct SendGridSettingsModel {
    pub send_grid_api_key: String,
    pub seq_conn_string: String,
    pub from_email: String,
}
