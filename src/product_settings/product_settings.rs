use crate::{
    BinanceFeedSettings, BrandSettings, PriceBridgesSettings, RecaptchaSettings,
    SendGridSettingsModel, TrackBoxSettingsModel,
};

service_sdk::macros::use_my_no_sql_entity!();

#[enum_of_my_no_sql_entity(table_name:"product-settings", generate_unwraps)]
pub enum ProductSettings {
    SendGrid(SendGridSettingsModel),
    TrackBox(TrackBoxSettingsModel),
    PriceBridgesSettings(PriceBridgesSettings),
    BinanceFeedSettings(BinanceFeedSettings),
    RecaptchaSettings(RecaptchaSettings),
    BrandSettings(BrandSettings),
}
