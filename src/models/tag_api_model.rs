#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TagApiModel {
    pub key: String,
    pub value: String,
}
