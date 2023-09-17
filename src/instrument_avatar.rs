use serde::{Deserialize, Serialize};
pub use service_sdk::my_no_sql_sdk;
#[my_no_sql_sdk::macros::my_no_sql_entity("instrumentsavatar")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentAvatarMyNoSqlEntity {
    #[serde(rename = "Avatar")]
    pub avatar: String,
}

impl InstrumentAvatarMyNoSqlEntity {
    pub fn get_instrument_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_file_type(&self) -> &str {
        &self.row_key
    }
}
