use crate::{
    error::CoreError,
    sandbox::{Sandbox, SandboxCommand, SandboxExecuteOptions, SandboxResultStatus},
    utils::get_binary_path::get_binary_path,
};

use super::{
    ExecutionErrorOutput, ExecutionParams, ExecutionResult, ExecutionSuccessOutput,
    LanguageExecutor,
};

pub struct PythonExecutor {}

impl LanguageExecutor for PythonExecutor {
    async fn run(
        &self,
        params: &ExecutionParams<'_>,
        sandbox: &impl Sandbox,
    ) -> Result<ExecutionResult, CoreError> {
        sandbox.add_file("./main.py", params.code).await;

        let binary = get_binary_path("python3").await;

        let execute_result = sandbox
            .execute(
                &SandboxCommand {
                    binary: &binary,
                    args: vec!["./main.py"],
                },
                &SandboxExecuteOptions::Sandboxed {
                    stdin: params.stdin,
                },
            )
            .await?;

        match execute_result.status {
            SandboxResultStatus::TimeLimitExceeded => {
                Result::Ok(ExecutionResult::TimeLimitExceeded)
            }
            SandboxResultStatus::MemoryLimitExceeded => {
                Result::Ok(ExecutionResult::MemoryLimitExceeded)
            }
            SandboxResultStatus::RuntimeError => {
                Result::Ok(ExecutionResult::RuntimeError(ExecutionErrorOutput {
                    stdout: execute_result.stdout,
                    stderr: execute_result.stderr,
                }))
            }
            SandboxResultStatus::Success => {
                Result::Ok(ExecutionResult::Success(ExecutionSuccessOutput {
                    stdout: execute_result.stdout,
                    stderr: execute_result.stderr,
                    time: execute_result.time,
                    memory: execute_result.memory,
                }))
            }
        }
    }
}
