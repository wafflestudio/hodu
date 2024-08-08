use crate::{
    sandbox::{Sandbox, SandboxCommand, SandboxExecuteOptions},
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

        if !compile_result.success {
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

        if !execute_result.success {
            return ExecutionResult::RuntimeError(ExecutionErrorOutput {
                stdout: execute_result.stdout,
                stderr: execute_result.stderr,
            });
        }

        ExecutionResult::Success(ExecutionSuccessOutput {
            stdout: execute_result.stdout,
            stderr: execute_result.stderr,
            time: execute_result.time,
            memory: execute_result.memory,
        })
    }
}
