use crate::sandbox::{ExecutionCommand, ExecutionResult, Sandbox};

use super::LanguageExecutor;

pub struct CppExecutor {}

impl LanguageExecutor for CppExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult {
        sandbox.add_file("./main.cpp", code).await;

        let compile_result = sandbox
            .execute(
                ExecutionCommand {
                    binary: "g++",
                    args: vec!["-o", "./main", "./main.cpp"],
                },
                false,
            )
            .await;

        if !compile_result.success {
            return compile_result;
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
