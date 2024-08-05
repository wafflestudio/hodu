use crate::sandbox::isolate::execute_isolate;

use super::{ExecutionCommand, ExecutionParams, ExecutionResult};

pub async fn run_java_code(code: &str) -> ExecutionResult {
    execute_isolate(ExecutionParams {
        code: code.to_string(),
        filename: "Main.java".to_string(),
        compile_command: Some(ExecutionCommand {
            binary: "/usr/lib/jvm/java-17-openjdk-arm64/bin/javac".to_string(),
            args: vec!["./Main.java".to_string()],
        }),
        execute_command: ExecutionCommand {
            binary: "/usr/lib/jvm/java-17-openjdk-arm64/bin/java".to_string(),
            args: vec!["Main".to_string()],
        },
    })
    .await
}
