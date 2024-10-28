#[my_no_sql_entity("integration-creads")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntegrationCreadsNoSqlEntity {
    pub id: String,
    pub name: String,
    pub secret_key: String,
}
