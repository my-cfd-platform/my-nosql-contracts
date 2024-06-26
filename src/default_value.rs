use serde::*;
service_sdk::macros::use_my_no_sql_entity!();
// cSpell:disable
#[my_no_sql_entity("defaultvalues")]
// cSpell:enable
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
        // cSpell:disable
        "defaulttg"
        // cSpell:enable
    }

    pub fn row_key_as_fav_instrument_web() -> &'static str {
        // cSpell:disable
        "favouriteinstrumentsweb"
        // cSpell:enable
    }

    pub fn row_key_as_fav_instrument_mobile() -> &'static str {
        // cSpell:disable
        "favouriteinstrumentsmobile"
        // cSpell:enable
    }

    pub fn row_key_as_default_instrument_png_avatar() -> &'static str {
        // cSpell:disable
        "TradingInstrumentAvatarPng"
        // cSpell:enable
    }

    pub fn row_key_as_default_instrument_svg_avatar() -> &'static str {
        // cSpell:disable
        "TradingInstrumentAvatarSvg"
        // cSpell:enable
    }

    pub fn row_key_as_available_collaterals() -> &'static str {
        // cSpell:disable
        "CollaterallCurrencies"
        // cSpell:enable
    }

    pub fn row_key_as_default_trading_group() -> &'static str {
        // cSpell:disable
        "TradingGroup"
        // cSpell:enable
    }

    pub const ROW_KEY_SEQUENCE_ACCOUNT_ID: &'static str = "SeqAccountId";

    pub fn get_asset_id(&self) -> &str {
        &self.row_key
    }
}
