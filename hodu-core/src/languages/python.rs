use crate::{sandbox::isolate::execute_isolate, utils::realpath::realpath};

use super::{ExecutionCommand, ExecutionParams, ExecutionResult};

pub async fn run_python_code(code: &str) -> ExecutionResult {
    let python3 = realpath("python3").await;

    execute_isolate(ExecutionParams {
        code: code.to_string(),
        filename: "main.py".to_string(),
        compile_command: None,
        execute_command: ExecutionCommand {
            binary: python3,
            args: vec!["./main.py".to_string()],
        },
    })
    .await
}
