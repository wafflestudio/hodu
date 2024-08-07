pub mod error;
mod languages;
mod sandbox;
mod utils {
    pub mod get_binary_path;
}

use languages::{
    c::CExecutor, cpp::CppExecutor, java::JavaExecutor, javascript::JavaScriptExecutor,
    python::PythonExecutor, ExecutionResult, LanguageExecutor,
};
use sandbox::{isolate::IsolateSandbox, ulimit::UlimitSandbox, Sandbox, SandboxSpecification};

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
    pub time: f64,
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
    let sandbox = UlimitSandbox::create(SandboxSpecification {
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

    MarkResult {
        status: match &execute_result {
            // TODO: implement
            ExecutionResult::Success(_) => MarkResultStatus::Correct,
            ExecutionResult::CompileError(_) => MarkResultStatus::CompileError,
        },
        time: match &execute_result {
            ExecutionResult::Success(result) => result.time,
            ExecutionResult::CompileError(_) => 0.0,
        },
        stdout: match &execute_result {
            ExecutionResult::Success(result) => result.stdout.clone(),
            ExecutionResult::CompileError(result) => result.stdout.clone(),
        },
        stderr: match &execute_result {
            ExecutionResult::Success(result) => result.stderr.clone(),
            ExecutionResult::CompileError(result) => result.stderr.clone(),
        },
    }
}
