use super::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_of_my_no_sql_entity(table_name:"product-settings", generate_unwraps)]
pub enum ProductSettings {
    SendGrid(SendGridSettingsModel),
    TrackBox(TrackBoxSettingsModel),
    PriceBridgesSettings(PriceBridgesSettings),
    RecaptchaSettings(RecaptchaSettings),
    BrandSettings(BrandSettings),
    YbABookSettings(YbABookSettings),
    YbPriceFeedSettings(YbPriceFeedSettings),
    WlPriceFeedSettings(WlPriceFeedSettings),
}
