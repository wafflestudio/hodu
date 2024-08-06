use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CodeSubmission {
    #[serde(default = "default_id")]
    pub id: String,
    pub language: Language,
    pub code: String,
}

fn default_id() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Deserialize, Debug)]
pub enum Language {
    #[serde(rename = "c")]
    C,
    #[serde(rename = "c++")]
    Cpp,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "javascript")]
    Javascript,
    #[serde(rename = "python")]
    Python,
}
