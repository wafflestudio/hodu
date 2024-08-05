use rand::Rng;
use std::path::PathBuf;
use tokio::process::Command;

use super::{ExecutionCommand, ExecutionResult, Sandbox, SandboxEnvironment};

pub struct Isolate {
    box_id: i32,
    path: PathBuf,
    memory_limit: u32,
    time_limit: u32,
}

impl Sandbox for Isolate {
    async fn create(environment: SandboxEnvironment) -> Self {
        let box_id = rand::thread_rng().gen_range(0..1000);
        let init_output = Command::new("isolate")
            .arg(format!("--box-id={}", box_id))
            .arg("--init")
            .output()
            .await
            .expect("Failed to init box");

        let working_directory = format!(
            "{}/box",
            std::str::from_utf8(&init_output.stdout)
                .expect("Invalid output")
                .trim()
        );

        Isolate {
            box_id,
            path: PathBuf::from(working_directory),
            memory_limit: environment.memory_limit,
            time_limit: environment.time_limit,
        }
    }

    async fn add_file(&self, filename: &str, content: &str) {
        let source_path = format!("{}/{}", self.path.to_str().unwrap().trim(), filename);

        std::fs::write(source_path, content).expect("Failed to write file");
    }

    async fn execute(&self, command: ExecutionCommand<'_>, sandboxed: bool) -> ExecutionResult {
        let run_output = match sandboxed {
            true => Command::new("isolate")
                .arg(format!("--box-id={}", self.box_id))
                .arg("--run")
                .arg(format!("--processes={}", 128))
                .arg(format!("--time={}", self.time_limit))
                .arg(format!("--mem={}", self.memory_limit))
                .arg(command.binary)
                .args(&command.args)
                .output()
                .await
                .expect("Failed to execute"),
            false => Command::new(command.binary)
                .args(&command.args)
                .current_dir(&self.path)
                .output()
                .await
                .expect("Failed to execute"),
        };

        ExecutionResult {
            output: match run_output.status.success() {
                true => std::str::from_utf8(&run_output.stdout)
                    .expect("Invalid output")
                    .to_string(),

                false => std::str::from_utf8(&run_output.stderr)
                    .expect("Invalid output")
                    .to_string(),
            },
            success: run_output.status.success(),
        }
    }

    async fn destroy(&self) {
        Command::new("isolate")
            .arg(format!("--box-id={}", self.box_id))
            .arg("--cleanup")
            .output()
            .await
            .expect("Failed to cleanup box");
    }
}
