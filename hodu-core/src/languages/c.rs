use super::{ExecutionCommand, ExecutionParams, ExecutionResult};
use crate::sandbox::isolate::execute_isolate;

pub async fn run_c_code(code: &str) -> ExecutionResult {
    execute_isolate(ExecutionParams {
        code: code.to_string(),
        filename: "./main.c".to_string(),
        compile_command: Some(ExecutionCommand {
            binary: "gcc".to_string(),
            args: vec![
                "-o".to_string(),
                "./main".to_string(),
                "./main.c".to_string(),
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
}
