use serde::{Deserialize, Serialize};

pub mod c;
pub mod java;

#[derive(Deserialize)]
pub enum Language {
    C,
    JAVA,
}

#[derive(Serialize)]
pub struct ExecutionResult {
    stdout: String,
    stderr: String,
    success: bool,
}