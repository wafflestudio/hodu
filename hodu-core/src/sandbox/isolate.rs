use rand::Rng;
use std::path::PathBuf;
use tokio::process::Command;

use super::{Sandbox, SandboxCommand, SandboxExecuteOptions, SandboxResult, SandboxSpecification};

pub struct Isolate {
    box_id: i32,
    path: PathBuf,
    memory_limit: u32,
    time_limit: u32,
}

impl Sandbox for Isolate {
    async fn create(environment: SandboxSpecification) -> Self {
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

    async fn execute(
        &self,
        command: &SandboxCommand<'_>,
        options: &SandboxExecuteOptions<'_>,
    ) -> SandboxResult {
        let (output, time, memory) = match options {
            SandboxExecuteOptions::Sandboxed { stdin } => {
                let home_dir = self.path.to_str().unwrap().trim();

                std::fs::write(format!("{}/stdin.txt", home_dir), stdin)
                    .expect("Failed to write file");

                let result = Command::new("isolate")
                    .arg(format!("--box-id={}", self.box_id))
                    .arg(format!("--processes={}", 128))
                    .arg(format!("--time={}", self.time_limit))
                    .arg(format!("--mem={}", self.memory_limit))
                    .arg(format!("--meta={}/meta.txt", home_dir))
                    .arg("--stdin=stdin.txt")
                    .arg("--run")
                    .arg(command.binary)
                    .args(&command.args)
                    .output()
                    .await
                    .expect("Failed to execute");

                let meta_content = std::fs::read_to_string(format!("{}/meta.txt", home_dir))
                    .expect("Failed to read file");

                let time = meta_content
                    .lines()
                    .find(|line| line.starts_with("time-wall:"))
                    .and_then(|line| line.split(':').nth(1))
                    .map(|value| value.trim().parse::<f64>())
                    .unwrap_or(Ok(0.0))
                    .expect("Failed to parse time");

                let memory = meta_content
                    .lines()
                    .find(|line| line.starts_with("max-rss:"))
                    .and_then(|line| line.split(':').nth(1))
                    .map(|value| value.trim().parse::<u32>())
                    .unwrap_or(Ok(0))
                    .expect("Failed to parse memory");

                std::fs::remove_file(format!("{}/stdin.txt", home_dir))
                    .expect("Failed to remove file");
                std::fs::remove_file(format!("{}/meta.txt", home_dir))
                    .expect("Failed to remove file");

                (result, time, memory)
            }
            SandboxExecuteOptions::Unsandboxed => (
                Command::new(command.binary)
                    .args(&command.args)
                    .current_dir(&self.path)
                    .output()
                    .await
                    .expect("Failed to execute"),
                0.0,
                0,
            ),
        };

        SandboxResult {
            stdout: match output.status.success() {
                true => String::from_utf8(output.stdout.clone()).expect("Invalid output"),
                false => String::new(),
            },
            stderr: match output.status.success() {
                true => String::new(),
                false => String::from_utf8(output.stderr.clone()).expect("Invalid output"),
            },
            success: output.status.success(),
            time,
            memory,
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
