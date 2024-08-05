use rand::Rng;
use tokio::process::Command;

use crate::languages::{ExecutionParams, ExecutionResult};

pub async fn execute_isolate(params: ExecutionParams) -> ExecutionResult {
    let box_id = rand::thread_rng().gen_range(0..1000);
    let box_id_arg = format!("--box-id={}", box_id);

    let init_output = Command::new("isolate")
        .arg(&box_id_arg)
        .arg("--init")
        .output()
        .await
        .expect("Failed to init box");

    let working_directory = format!(
        "{}/box",
        std::str::from_utf8(&init_output.stdout)
            .expect("Invalid output")
            .trim(),
    );
    let source_path = format!("{}/{}", working_directory, params.filename);

    std::fs::write(source_path, params.code).expect("Failed to write file");

    if let Some(compile_command) = params.compile_command {
        let compile_output = Command::new(compile_command.binary)
            .current_dir(&working_directory)
            .args(&compile_command.args)
            .output()
            .await
            .expect("Failed to compile");

        if !compile_output.status.success() {
            return ExecutionResult {
                stdout: String::new(),
                stderr: String::from_utf8(compile_output.stderr).expect("Invalid runtime error"),
                success: false,
            };
        }
    }

    let run_output = Command::new("isolate")
        .arg(&box_id_arg)
        .arg("--run")
        .arg("-p128")
        .arg(params.execute_command.binary)
        .args(&params.execute_command.args)
        .output()
        .await
        .expect("Failed to execute");

    Command::new("isolate")
        .arg(&box_id_arg)
        .arg("--cleanup")
        .output()
        .await
        .expect("Failed to cleanup box");

    if !run_output.status.success() {
        return ExecutionResult {
            stdout: String::new(),
            stderr: String::from_utf8(run_output.stderr).expect("Invalid runtime error"),
            success: false,
        };
    }

    ExecutionResult {
        stdout: String::from_utf8(run_output.stdout).expect("Invalid output"),
        stderr: String::new(),
        success: true,
    }
}
