#[my_no_sql_entity("swap-profile")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SwapProfileNoSqlModel {
    pub id: String,
    pub instrument_id: String,
    pub long: f64,
    pub short: f64,
}
