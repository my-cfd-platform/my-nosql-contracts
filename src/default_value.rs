use serde::*;

#[my_no_sql_macros::my_no_sql_entity("defaultvalues")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DefaultsNoSqlEntity {
    pub value: Option<String>,
    pub values: Option<Vec<String>>,
}

impl DefaultsNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "dv"
    }

    pub fn generate_default_tg_rk() -> &'static str {
        "defaulttg"
    }

    pub fn row_key_as_fav_instrument_web() -> &'static str {
        "favouriteinstrumentsweb"
    }

    pub fn row_key_as_fav_instrument_mobile() -> &'static str {
        "favouriteinstrumentsmobile"
    }

    pub fn get_asset_id(&self) -> &str {
        &self.row_key
    }
}
