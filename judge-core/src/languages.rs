use serde::{Deserialize, Serialize};

pub mod c;
pub mod cpp;
pub mod java;
pub mod javascript;
pub mod python;

#[derive(Deserialize)]
pub enum Language {
    C,
    CPP,
    JAVA,
    PYTHON,
    JAVASCRIPT,
}

#[derive(Serialize)]
pub struct ExecutionResult {
    stdout: String,
    stderr: String,
    success: bool,
}
