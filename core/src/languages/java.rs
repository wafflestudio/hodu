use std::process::Command;

use super::ExecutionResult;

pub fn run_java_code(code: &str, temp_dir: &std::path::Path) -> ExecutionResult {
    let temp_java_file = temp_dir.join("Main.java");
    let temp_class_file = temp_dir.join("Main.class");

    // Java 코드를 temp_dir/Main.java 파일로 저장합니다.
    std::fs::write(&temp_java_file, code).expect("Unable to write file");

    // Java 코드를 컴파일합니다.
    let compile_output = Command::new("javac")
        .arg(&temp_java_file)
        .output()
        .expect("Failed to compile Java code");

    if !compile_output.status.success() {
        return ExecutionResult {
            stdout: String::new(),
            stderr: String::from_utf8(compile_output.stderr).expect("Invalid compile error"),
            success: false,
        };
    }

    // 컴파일된 Java 코드를 실행합니다.
    let output = Command::new("java")
        .arg("-cp")
        .arg(temp_dir)
        .arg("Main")
        .output()
        .expect("Failed to execute Java code");

    if !output.status.success() {
        return ExecutionResult {
            stdout: String::new(),
            stderr: String::from_utf8(output.stderr).expect("Invalid runtime error"),
            success: false,
        };
    }

    // 임시 파일을 삭제합니다.
    std::fs::remove_file(&temp_java_file).expect("Unable to delete file");
    std::fs::remove_file(&temp_class_file).expect("Unable to delete file");

    ExecutionResult {
        stdout: String::from_utf8(output.stdout).expect("Invalid output"),
        stderr: String::new(),
        success: true,
    }
}
