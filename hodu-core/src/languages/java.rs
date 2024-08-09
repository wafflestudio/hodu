use crate::{
    error::CoreError,
    sandbox::{Sandbox, SandboxCommand, SandboxExecuteOptions, SandboxResultStatus},
    utils::get_binary_path::get_binary_path,
};

use super::{
    ExecutionErrorOutput, ExecutionParams, ExecutionResult, ExecutionSuccessOutput,
    LanguageExecutor,
};

pub struct JavaExecutor {}

impl LanguageExecutor for JavaExecutor {
    async fn run(
        &self,
        params: &ExecutionParams<'_>,
        sandbox: &impl Sandbox,
    ) -> Result<ExecutionResult, CoreError> {
        sandbox.add_file("./Main.java", params.code).await;

        let java = get_binary_path("java").await;

        let compile_result = sandbox
            .execute(
                &SandboxCommand {
                    binary: "javac",
                    args: vec!["./Main.java"],
                },
                &SandboxExecuteOptions::Unsandboxed,
            )
            .await?;

        if compile_result.status != SandboxResultStatus::Success {
            return Result::Ok(ExecutionResult::CompileError(ExecutionErrorOutput {
                stdout: compile_result.stdout,
                stderr: compile_result.stderr,
            }));
        }

        let execute_result = sandbox
            .execute(
                &SandboxCommand {
                    binary: &java,
                    args: vec!["Main"],
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
