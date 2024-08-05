use crate::{error::HoduCoreError, sandbox::isolate::execute_isolate, utils::get_binary_path::get_binary_path};

use super::{ExecutionCommand, ExecutionParams, ExecutionResult};

pub async fn run_java_code(code: &str) -> Result<ExecutionResult, HoduCoreError> {
    let java = get_binary_path("java").await;
    let javac = get_binary_path("javac").await;

    execute_isolate(ExecutionParams {
        code: code.to_string(),
        filename: "Main.java".to_string(),
        compile_command: Some(ExecutionCommand {
            binary: javac,
            args: vec!["./Main.java".to_string()],
        }),
        execute_command: ExecutionCommand {
            binary: java,
            args: vec!["Main".to_string()],
        },
    })
    .await
    .map_err(HoduCoreError::IsolateError)
}
