use serde::*;

service_sdk::macros::use_my_no_sql_entity!();
#[enum_model(partition_key:"settings", row_key: "brand")]
#[derive(Serialize, Deserialize, Clone)]
pub struct BrandSettings {
    #[serde(rename = "logoUrl")]
    pub logo_url: String,
    #[serde(rename = "policyUrl")]
    pub policy_url: String,
    #[serde(rename = "termsUrl")]
    pub terms_url: String,
    #[serde(rename = "withdrawFaqUrl")]
    pub withdraw_faq_url: String,
    #[serde(rename = "aboutUrl")]
    pub about_url: String,
    #[serde(rename = "supportUrl")]
    pub support_url: String,

    #[serde(rename = "brandName")]
    pub brand_name: String,

    #[serde(rename = "brandCopyrights")]
    pub brand_copyrights: String,
    #[serde(rename = "gaAsAccount")]
    pub ga_as_account: String,

    #[serde(rename = "mixPanelToken")]
    pub mix_panel_token: Option<String>,

    #[serde(rename = "faviconUrl")]
    pub favicon_url: String,

    #[serde(rename = "androidAppId")]
    pub android_app_id: Option<String>,
    #[serde(rename = "androidAppLink")]
    pub android_app_link: Option<String>,

    #[serde(rename = "iosAppId")]
    pub ios_app_id: Option<String>,
    #[serde(rename = "iosAppLink")]
    pub ios_app_link: Option<String>,

    #[serde(rename = "mobileAppLogo")]
    pub mobile_app_logo: Option<String>,
}
