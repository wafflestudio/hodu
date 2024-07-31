use crate::ExecutionResult;
use tokio::process::Command;

pub async fn run_java_code(code: &str, temp_dir: &std::path::Path) -> ExecutionResult {
    let temp_java_file = temp_dir.join("Main.java");

    std::fs::write(&temp_java_file, code).expect("Unable to write file");

    let compile_output = Command::new("javac")
        .arg(&temp_java_file)
        .output()
        .await
        .expect("Failed to compile Java code");

    if !compile_output.status.success() {
        return ExecutionResult {
            stdout: String::new(),
            stderr: String::from_utf8(compile_output.stderr).expect("Invalid compile error"),
            success: false,
        };
    }

    let output = Command::new("java")
        .arg("-cp")
        .arg(temp_dir)
        .arg("Main")
        .output()
        .await
        .expect("Failed to execute Java code");

    if !output.status.success() {
        return ExecutionResult {
            stdout: String::new(),
            stderr: String::from_utf8(output.stderr).expect("Invalid runtime error"),
            success: false,
        };
    }

    ExecutionResult {
        stdout: String::from_utf8(output.stdout).expect("Invalid output"),
        stderr: String::new(),
        success: true,
    }
}
