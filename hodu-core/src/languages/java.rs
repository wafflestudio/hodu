use crate::{
    sandbox::{Sandbox, SandboxCommand},
    utils::get_binary_path::get_binary_path,
};

use super::{ExecutionErrorOutput, ExecutionResult, ExecutionSuccessOutput, LanguageExecutor};

pub struct JavaExecutor {}

impl LanguageExecutor for JavaExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult {
        sandbox.add_file("./Main.java", code).await;

        let java = get_binary_path("java").await;

        let compile_result = sandbox
            .execute(
                SandboxCommand {
                    binary: "javac",
                    args: vec!["./Main.java"],
                },
                false,
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
                SandboxCommand {
                    binary: &java,
                    args: vec!["Main"],
                },
                true,
            )
            .await;

        ExecutionResult::Success(ExecutionSuccessOutput {
            stdout: execute_result.stdout,
            stderr: execute_result.stderr,
            time: execute_result.time,
        })
    }
}
