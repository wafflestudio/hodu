use crate::{
    sandbox::{ExecutionCommand, ExecutionResult, Sandbox},
    utils::get_binary_path::get_binary_path,
};

use super::LanguageExecutor;

pub struct PythonExecutor {}

impl LanguageExecutor for PythonExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult {
        sandbox.add_file("./main.py", code).await;

        let binary = get_binary_path("python3").await;

        let execute_result = sandbox
            .execute(
                ExecutionCommand {
                    binary: &binary,
                    args: vec!["./main.py"],
                },
                true,
            )
            .await;

        execute_result
    }
}
