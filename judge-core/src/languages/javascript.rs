use tokio::process::Command;

use super::ExecutionResult;

// TODO: isolate
pub async fn run_javascript_code(code: &str, temp_dir: &std::path::PathBuf) -> ExecutionResult {
    let source_path = temp_dir.join("main.js");

    std::fs::write(&source_path, code).expect("Unable to write file");

    let output = Command::new("node")
        .arg(&source_path)
        .output()
        .await
        .expect("Failed to execute JavaScript code");

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
