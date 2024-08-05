use crate::{sandbox::isolate::execute_isolate, utils::realpath::realpath};

use super::{ExecutionCommand, ExecutionParams, ExecutionResult};

pub async fn run_javascript_code(code: &str) -> ExecutionResult {
    let node = realpath("node").await;

    execute_isolate(ExecutionParams {
        code: code.to_string(),
        filename: "main.mjs".to_string(),
        compile_command: None,
        execute_command: ExecutionCommand {
            binary: node,
            args: vec!["./main.mjs".to_string()],
        },
    })
    .await
}
