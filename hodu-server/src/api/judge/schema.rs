use hodu_core::Language;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CodeSubmission {
    pub language: Language,
    pub code: String,
}
