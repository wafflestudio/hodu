use serde::{Deserialize, Serialize};

use crate::error::HoduCoreError;

mod c;
mod cpp;
mod java;
mod javascript;
mod python;

#[derive(Deserialize, Debug)]
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

pub async fn mark_code(language: &Language, code: String) -> Result<ExecutionResult, HoduCoreError> {
    match language {
        Language::C => c::run_c_code(&code).await,
        Language::CPP => cpp::run_cpp_code(&code).await,
        Language::JAVA => java::run_java_code(&code).await,
        Language::PYTHON => python::run_python_code(&code).await,
        Language::JAVASCRIPT => javascript::run_javascript_code(&code).await,
    }
}
