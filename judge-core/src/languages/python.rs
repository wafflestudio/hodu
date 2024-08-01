use crate::sandbox::isolate::execute_isolate;

use super::ExecutionResult;

pub async fn run_python_code(code: &str, temp_dir: &std::path::PathBuf) -> ExecutionResult {
    let source_path = temp_dir.join("main.py");

    std::fs::write(&source_path, code).expect("Unable to write file");

    let output = execute_isolate(
        temp_dir,
        &std::path::PathBuf::from("/usr/bin/python3.11"),
        &[source_path],
    );

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
