use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("instrumentsavatar")]
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
