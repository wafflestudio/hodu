use crate::{
    sandbox::{Sandbox, SandboxCommand, SandboxExecuteOptions, SandboxResultStatus},
    utils::get_binary_path::get_binary_path,
};

use super::{
    ExecutionErrorOutput, ExecutionParams, ExecutionResult, ExecutionSuccessOutput,
    LanguageExecutor,
};

pub struct JavaExecutor {}

impl LanguageExecutor for JavaExecutor {
    async fn run(&self, params: &ExecutionParams<'_>, sandbox: &impl Sandbox) -> ExecutionResult {
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
            .await;

        if compile_result.status != SandboxResultStatus::Success {
            return ExecutionResult::CompileError(ExecutionErrorOutput {
                stdout: compile_result.stdout,
                stderr: compile_result.stderr,
            });
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
            .await;

        match execute_result.status {
            SandboxResultStatus::TimeLimitExceeded => ExecutionResult::TimeLimitExceeded,
            SandboxResultStatus::MemoryLimitExceeded => ExecutionResult::MemoryLimitExceeded,
            SandboxResultStatus::RuntimeError => {
                ExecutionResult::RuntimeError(ExecutionErrorOutput {
                    stdout: execute_result.stdout,
                    stderr: execute_result.stderr,
                })
            }
            SandboxResultStatus::InternalError => ExecutionResult::InternalError,
            SandboxResultStatus::Success => ExecutionResult::Success(ExecutionSuccessOutput {
                stdout: execute_result.stdout,
                stderr: execute_result.stderr,
                time: execute_result.time,
                memory: execute_result.memory,
            }),
        }
    }
}
