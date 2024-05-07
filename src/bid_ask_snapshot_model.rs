use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("bidask-snapshots")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BidAskSnapshotNoSqlEntity {
    pub unix_timestamp_with_milis: u64,
    pub bid: f64,
    pub ask: f64,
    pub base: String,
    pub quote: String,
}

impl BidAskSnapshotNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "instruments_snapshot"
    }

    pub fn generate_row_key(instrument_id: &'static str) -> &'static str {
        instrument_id
    }

    pub fn get_instrument_id(&self) -> &str {
        &self.row_key
    }
}
