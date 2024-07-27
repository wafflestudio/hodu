use crate::ExecutionResult;
use std::process::Command;

pub fn run_c_code(code: &str, temp_dir: &std::path::Path) -> ExecutionResult {
    let temp_c_dir = temp_dir.join("example.c");
    let temp_exec_dir = temp_dir.join("example");

    std::fs::write(&temp_c_dir, code).expect("Unable to write file");

    let compile_output = Command::new("gcc")
        .arg(&temp_c_dir)
        .arg("-o")
        .arg(&temp_exec_dir)
        .output()
        .expect("Failed to compile C code");

    if !compile_output.status.success() {
        return ExecutionResult {
            stdout: String::new(),
            stderr: String::from_utf8(compile_output.stderr).expect("Invalid compile error"),
            success: false,
        };
    }

    let output = Command::new(&temp_exec_dir)
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
