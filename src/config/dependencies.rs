#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Dependencies {
    pub git: String,
    pub version: String,
}
