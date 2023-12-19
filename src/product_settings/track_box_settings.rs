use serde::*;

service_sdk::macros::use_my_no_sql_entity!();
#[enum_model(partition_key:"settings", row_key: "trackbox")]
#[derive(Serialize, Deserialize, Clone)]
pub struct TrackBoxSettingsModel {
    pub api_keys: Vec<String>,
}

impl TrackBoxSettingsModel {
    pub fn verify_track_box_key(&self, key: &String) -> bool {
        self.api_keys.contains(key)
    }
}
