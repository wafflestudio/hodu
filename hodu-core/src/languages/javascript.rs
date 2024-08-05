use crate::{
    sandbox::{Sandbox, SandboxCommand},
    utils::get_binary_path::get_binary_path,
};

use super::{ExecutionResult, ExecutionSuccessOutput, LanguageExecutor};

pub struct JavaScriptExecutor {}

impl LanguageExecutor for JavaScriptExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult {
        sandbox.add_file("./main.mjs", code).await;

        let node = get_binary_path("node").await;

        let execute_result = sandbox
            .execute(
                SandboxCommand {
                    binary: &node,
                    args: vec!["./main.mjs"],
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
