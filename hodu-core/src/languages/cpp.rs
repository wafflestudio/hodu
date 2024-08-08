use crate::sandbox::{Sandbox, SandboxCommand, SandboxExecuteOptions};

use super::{
    ExecutionErrorOutput, ExecutionParams, ExecutionResult, ExecutionSuccessOutput,
    LanguageExecutor,
};

pub struct CppExecutor {}

impl LanguageExecutor for CppExecutor {
    async fn run(&self, params: &ExecutionParams<'_>, sandbox: &impl Sandbox) -> ExecutionResult {
        sandbox.add_file("./main.cpp", params.code).await;

        let compile_result = sandbox
            .execute(
                &SandboxCommand {
                    binary: "g++",
                    args: vec!["-o", "./main", "./main.cpp"],
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
                    binary: "./main",
                    args: vec![],
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
