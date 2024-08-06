use crate::{error::HoduCoreError, sandbox::isolate::execute_isolate};

use super::{ExecutionCommand, ExecutionParams, ExecutionResult};

pub async fn run_cpp_code(code: &str) -> Result<ExecutionResult, HoduCoreError> {
    execute_isolate(ExecutionParams {
        code: code.to_string(),
        filename: "main.cpp".to_string(),
        compile_command: Some(ExecutionCommand {
            binary: "g++".to_string(),
            args: vec![
                "-o".to_string(),
                "./main".to_string(),
                "./main.cpp".to_string(),
            ],
        }),
        execute_command: {
            ExecutionCommand {
                binary: "./main".to_string(),
                args: vec![],
            }
        },
    })
    .await
    .map_err(HoduCoreError::IsolateError)
}
