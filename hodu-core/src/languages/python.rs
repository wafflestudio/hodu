use crate::{
    sandbox::{Sandbox, SandboxCommand},
    utils::get_binary_path::get_binary_path,
};

use super::{ExecutionResult, ExecutionSuccessOutput, LanguageExecutor};

pub struct PythonExecutor {}

impl LanguageExecutor for PythonExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult {
        sandbox.add_file("./main.py", code).await;

        let binary = get_binary_path("python3").await;

        let execute_result = sandbox
            .execute(
                SandboxCommand {
                    binary: &binary,
                    args: vec!["./main.py"],
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
