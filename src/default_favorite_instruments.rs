use serde::{Deserialize, Serialize};

// cSpell:disable
#[my_no_sql_macros::my_no_sql_entity("defaultvalues")]
// cSpell:enable
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DefaultFavoriteInstruments {
    pub instruments: Vec<String>,
}

impl DefaultFavoriteInstruments {
    pub fn generate_partition_key() -> &'static str {
        "fi"
    }

    pub fn generate_row_key_web() -> &'static str {
        "web"
    }

    pub fn generate_row_key_mobile() -> &'static str {
        "mobile"
    }
}
