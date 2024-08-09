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

const DEFAULT_COMPILE_OPTIONS: [&str; 7] = [
    "--release",
    "17",
    "-encoding",
    "UTF-8",
    "-J-Xms1024m",
    "-J-Xmx1920m",
    "-J-Xss512m",
]; // 덮어씌울 수 있다.
const DEFAULT_COMPILE_ARGS: [&str; 1] = ["./Main.java"]; // 덮어씌울 수 없다.

impl LanguageExecutor for JavaExecutor {
    async fn run(
        &self,
        params: &ExecutionParams<'_>,
        sandbox: &impl Sandbox,
    ) -> Result<ExecutionResult, CoreError> {
        sandbox.add_file("./Main.java", params.code).await;

        let java = get_binary_path("java").await;

        let mut compile_args = vec![];
        compile_args.extend(
            params
                .compile_options
                .clone()
                .unwrap_or(DEFAULT_COMPILE_OPTIONS.to_vec()),
        );
        compile_args.extend(&DEFAULT_COMPILE_ARGS.to_vec());

        let compile_result = sandbox
            .execute(
                &SandboxCommand {
                    binary: "javac",
                    args: compile_args,
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
