use serde::*;

service_sdk::macros::use_my_no_sql_entity!();
#[enum_model(partition_key:"settings", row_key: "recaptcha")]
#[derive(Serialize, Deserialize, Clone)]
pub struct RecaptchaSettings {
    pub public_key: String,
    pub secret_key: String,
    pub score_to_verify: f64,
    pub disabled: bool,
}
