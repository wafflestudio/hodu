use rand::Rng;

use std::process::Command;

use super::ExecutionResult;

pub fn run_c_code(code: &str, temp_dir: &std::path::Path) -> ExecutionResult {
    let source_path = temp_dir.join("example.c");
    let binary_path = temp_dir.join("example");
    let box_id = rand::thread_rng().gen_range(0..1000);

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

    let box_id_arg = format!("--box-id={}", box_id);
    Command::new("isolate")
        .arg(&box_id_arg)
        .arg("--init")
        .output()
        .expect("Failed to init box");
    let output = Command::new("isolate")
        .arg(format!("--dir={}", temp_dir.display()))
        .arg(&box_id_arg)
        .arg("--run")
        .arg("--")
        .arg(&binary_path)
        .output()
        .expect("Failed to execute C code");
    Command::new("isolate")
        .arg(&box_id_arg)
        .arg("--cleanup")
        .output()
        .expect("Failed to cleanup box");

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
