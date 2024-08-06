use hodu_core::Language;
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
