use crate::sandbox::{Sandbox, SandboxCommand};

use super::{ExecutionErrorOutput, ExecutionResult, ExecutionSuccessOutput, LanguageExecutor};

pub struct CppExecutor {}

impl LanguageExecutor for CppExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult {
        sandbox.add_file("./main.cpp", code).await;

        let compile_result = sandbox
            .execute(
                SandboxCommand {
                    binary: "g++",
                    args: vec!["-o", "./main", "./main.cpp"],
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
                    binary: "./main",
                    args: vec![],
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
