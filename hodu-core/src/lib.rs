pub mod error;
mod languages;
mod sandbox;
mod utils {
    pub mod get_binary_path;
}

use languages::{
    c::CExecutor, cpp::CppExecutor, java::JavaExecutor, javascript::JavaScriptExecutor,
    python::PythonExecutor, LanguageExecutor,
};
use sandbox::{isolate::Isolate, Sandbox, SandboxEnvironment};

pub enum Language {
    C,
    Cpp,
    Java,
    Python,
    JavaScript,
}

pub struct MarkParams<'a> {
    pub language: &'a Language,
    pub code: &'a str,
    pub stdin: &'a str,
    pub expected_stdout: &'a str,
    pub time_limit: u32,
    pub memory_limit: u32,
}

pub struct MarkResult {
    pub status: MarkResultStatus,
    pub time: u32,
    pub memory: u32,
    pub stdout: String,
    pub stderr: String,
}

pub enum MarkResultStatus {
    Correct,
    Wrong,
    CompileError,
    RuntimeError,
    TimeLimitExceeded,
    MemoryLimitExceeded,
}

pub async fn mark(params: MarkParams<'_>) -> MarkResult {
    let sandbox = Isolate::create(SandboxEnvironment {
        memory_limit: params.memory_limit,
        time_limit: params.time_limit,
    })
    .await;

    let execute_result = match params.language {
        Language::C => CExecutor {}.run(params.code, &sandbox).await,
        Language::Cpp => CppExecutor {}.run(params.code, &sandbox).await,
        Language::Java => JavaExecutor {}.run(params.code, &sandbox).await,
        Language::Python => PythonExecutor {}.run(params.code, &sandbox).await,
        Language::JavaScript => JavaScriptExecutor {}.run(params.code, &sandbox).await,
    };

    sandbox.destroy().await;

    // TODO: implement
    MarkResult {
        status: MarkResultStatus::Correct,
        time: 0,
        memory: 0,
        stdout: match &execute_result.success {
            true => execute_result.output.to_string(),
            false => String::new(),
        },
        stderr: match &execute_result.success {
            true => String::new(),
            false => execute_result.output.to_string(),
        },
    }
}
