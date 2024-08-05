use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct CodeSubmission {
    #[serde(default = "default_id")]
    pub id: String,
    pub language: String,
    pub code: String,
}

fn default_id() -> String {
    Uuid::new_v4().to_string()
}
