use crate::sandbox::isolate::execute_isolate;

use super::{ExecutionCommand, ExecutionParams, ExecutionResult};

pub async fn run_javascript_code(code: &str) -> ExecutionResult {
    execute_isolate(ExecutionParams {
        code: code.to_string(),
        filename: "main.mjs".to_string(),
        compile_command: None,
        execute_command: ExecutionCommand {
            binary: "/usr/bin/node".to_string(),
            args: vec!["./main.mjs".to_string()],
        },
    })
    .await
}
