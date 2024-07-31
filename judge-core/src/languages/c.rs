use std::process::Command;

use super::ExecutionResult;

pub fn run_c_code(code: &str, temp_dir: &std::path::Path) -> ExecutionResult {
    let source_path = temp_dir.join("example.c");
    let binary_path = temp_dir.join("example");

    std::fs::write(&source_path, code).expect("Unable to write file");

    let compile_output = Command::new("gcc")
        .arg(&source_path)
        .arg("-o")
        .arg(&binary_path)
        .output()
        .expect("Failed to compile C code");

    if !compile_output.status.success() {
        return ExecutionResult {
            stdout: String::new(),
            stderr: String::from_utf8(compile_output.stderr).expect("Invalid compile error"),
            success: false,
        };
    }

    let output = Command::new("isolate")
        .arg(format!("--dir={}", temp_dir.display()))
        .arg("--run")
        .arg("--")
        .arg(&binary_path)
        .output()
        .expect("Failed to execute C code");

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
