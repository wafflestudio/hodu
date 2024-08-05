use crate::sandbox::{ExecutionCommand, ExecutionResult, Sandbox};

use super::LanguageExecutor;

pub struct CExecutor {}

impl LanguageExecutor for CExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult {
        sandbox.add_file("./main.c", code).await;

        let compile_result = sandbox
            .execute(
                ExecutionCommand {
                    binary: "gcc",
                    args: vec!["-o", "./main", "./main.c"],
                },
                false,
            )
            .await;

        if !compile_result.success {
            return ExecutionResult {
                output: compile_result.output,
                success: false,
            };
        }

        let execute_result = sandbox
            .execute(
                ExecutionCommand {
                    binary: "./main",
                    args: vec![],
                },
                true,
            )
            .await;

        execute_result
    }
}
