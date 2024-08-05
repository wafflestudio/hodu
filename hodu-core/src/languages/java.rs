use crate::{
    sandbox::{ExecutionCommand, ExecutionResult, Sandbox},
    utils::get_binary_path::get_binary_path,
};

use super::LanguageExecutor;

pub struct JavaExecutor {}

impl LanguageExecutor for JavaExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult {
        sandbox.add_file("./Main.java", code).await;

        let java = get_binary_path("java").await;

        let compile_result = sandbox
            .execute(
                ExecutionCommand {
                    binary: "javac",
                    args: vec!["./Main.java"],
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
                    binary: &java,
                    args: vec!["Main"],
                },
                true,
            )
            .await;

        execute_result
    }
}
