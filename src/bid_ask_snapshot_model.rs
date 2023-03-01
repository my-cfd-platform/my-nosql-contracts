use serde::*;

#[my_no_sql_macros::my_no_sql_entity("bidask-snapshots")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BidAskSnapshotNoSqlEntity {
    pub unix_timestamp_with_milis: u64,
    pub bid: f64,
    pub ask: f64,
}

impl BidAskSnapshotNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "instruments_snapshot"
    }
}
