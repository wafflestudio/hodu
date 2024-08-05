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
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

pub struct ExecutionParams {
    pub code: String,
    pub filename: String,
    pub compile_command: Option<ExecutionCommand>,
    pub execute_command: ExecutionCommand,
}

pub struct ExecutionCommand {
    pub binary: String,
    pub args: Vec<String>,
}
