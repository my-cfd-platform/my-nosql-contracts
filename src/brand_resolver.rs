use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("brand-resolver")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrandResolverMyNoSqlEntity {
    pub brand_id: String,
}

impl BrandResolverMyNoSqlEntity {
    const PARTITION_KEY: &'static str = "b";

    pub fn get_domain(&self) -> &str {
        self.row_key.as_str()
    }
}
