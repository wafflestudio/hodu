use crate::{
    sandbox::{ExecutionCommand, ExecutionResult, Sandbox},
    utils::get_binary_path::get_binary_path,
};

use super::LanguageExecutor;

pub struct JavaScriptExecutor {}

impl LanguageExecutor for JavaScriptExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult {
        sandbox.add_file("./main.mjs", code).await;

        let node = get_binary_path("node").await;

        let execute_result = sandbox
            .execute(
                ExecutionCommand {
                    binary: &node,
                    args: vec!["./main.mjs"],
                },
                true,
            )
            .await;

        execute_result
    }
}
