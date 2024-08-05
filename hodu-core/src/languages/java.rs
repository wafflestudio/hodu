use crate::{sandbox::isolate::execute_isolate, utils::realpath::realpath};

use super::{ExecutionCommand, ExecutionParams, ExecutionResult};

pub async fn run_java_code(code: &str) -> ExecutionResult {
    let java = realpath("java").await;
    let javac = realpath("javac").await;

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
}
