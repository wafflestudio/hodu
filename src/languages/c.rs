use crate::ExecutionResult;
use std::process::Command;

pub fn run_c_code(code: &str, temp_dir: &std::path::Path) -> ExecutionResult {
    let temp_c_dir = temp_dir.join("example.c");
    let temp_exec_dir = temp_dir.join("example");

    // C 코드를 temp_dir/example.c 파일로 저장합니다.
    std::fs::write(&temp_c_dir, code).expect("Unable to write file");

    // C 코드를 컴파일합니다.
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

    // 컴파일된 C 코드를 실행합니다.
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

    // 임시 파일을 삭제합니다.
    std::fs::remove_file(&temp_c_dir).expect("Unable to delete file");
    std::fs::remove_file(&temp_exec_dir).expect("Unable to delete file");

    ExecutionResult {
        stdout: String::from_utf8(output.stdout).expect("Invalid output"),
        stderr: String::new(),
        success: true,
    }
}
