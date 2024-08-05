use crate::sandbox::isolate::execute_isolate;

use super::{ExecutionCommand, ExecutionParams, ExecutionResult};

pub async fn run_python_code(code: &str) -> ExecutionResult {
    execute_isolate(ExecutionParams {
        code: code.to_string(),
        filename: "main.py".to_string(),
        compile_command: None,
        execute_command: ExecutionCommand {
            binary: "/usr/bin/python3.11".to_string(),
            args: vec!["./main.py".to_string()],
        },
    })
    .await
}
