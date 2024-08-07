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
use sandbox::{linux_user::LinuxUserSandbox, Sandbox, SandboxSpecification};

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
    let sandbox = LinuxUserSandbox::create(SandboxSpecification {
        memory_limit: params.memory_limit,
        time_limit: params.time_limit,
    })
    .await;

    let run_params = languages::ExecutionParams {
        code: params.code,
        stdin: params.stdin,
    };

    let execute_result = match params.language {
        Language::C => CExecutor {}.run(&run_params, &sandbox).await,
        Language::Cpp => CppExecutor {}.run(&run_params, &sandbox).await,
        Language::Java => JavaExecutor {}.run(&run_params, &sandbox).await,
        Language::JavaScript => JavaScriptExecutor {}.run(&run_params, &sandbox).await,
        Language::Python => PythonExecutor {}.run(&run_params, &sandbox).await,
    };

    sandbox.destroy().await;

    MarkResult {
        status: match &execute_result {
            ExecutionResult::Success(result) => {
                if result.stdout.trim().eq(params.expected_stdout.trim()) {
                    MarkResultStatus::Correct
                } else {
                    MarkResultStatus::Wrong
                }
            }
            ExecutionResult::CompileError(_) => MarkResultStatus::CompileError,
            ExecutionResult::RuntimeError(_) => MarkResultStatus::RuntimeError,
        },
        time: match &execute_result {
            ExecutionResult::Success(result) => result.time,
            _ => 0.0,
        },
        stdout: match &execute_result {
            ExecutionResult::Success(result) => result.stdout.clone(),
            ExecutionResult::CompileError(result) => result.stdout.clone(),
            ExecutionResult::RuntimeError(result) => result.stdout.clone(),
        },
        stderr: match &execute_result {
            ExecutionResult::Success(result) => result.stderr.clone(),
            ExecutionResult::CompileError(result) => result.stderr.clone(),
            ExecutionResult::RuntimeError(result) => result.stderr.clone(),
        },
    }
}
