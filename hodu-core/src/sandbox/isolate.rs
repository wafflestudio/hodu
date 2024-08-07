use rand::Rng;
use regex::Regex;
use std::path::PathBuf;
use tokio::process::Command;

use super::{Sandbox, SandboxCommand, SandboxExecuteOptions, SandboxResult, SandboxSpecification};

pub struct IsolateSandbox {
    box_id: i32,
    path: PathBuf,
    memory_limit: u32,
    time_limit: u32,
}

impl Sandbox for IsolateSandbox {
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

        IsolateSandbox {
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
        let run_output = match options {
            SandboxExecuteOptions::Sandboxed { stdin } => {
                std::fs::write(
                    format!("{}/stdin.txt", self.path.to_str().unwrap().trim()),
                    stdin,
                )
                .expect("Failed to write file");

                let result = Command::new("isolate")
                    .arg(format!("--box-id={}", self.box_id))
                    .arg(format!("--processes={}", 128))
                    .arg(format!("--time={}", self.time_limit))
                    .arg(format!("--mem={}", self.memory_limit))
                    .arg("--stdin=stdin.txt")
                    .arg("--run")
                    .arg(command.binary)
                    .args(&command.args)
                    .output()
                    .await
                    .expect("Failed to execute");

                std::fs::remove_file(format!("{}/stdin.txt", self.path.to_str().unwrap().trim()))
                    .expect("Failed to remove file");

                result
            }
            SandboxExecuteOptions::Unsandboxed => Command::new(command.binary)
                .args(&command.args)
                .current_dir(&self.path)
                .output()
                .await
                .expect("Failed to execute"),
        };

        SandboxResult {
            stdout: match run_output.status.success() {
                true => String::from_utf8(run_output.stdout.clone()).expect("Invalid output"),
                false => String::new(),
            },
            stderr: match run_output.status.success() {
                true => String::new(),
                false => String::from_utf8(run_output.stderr.clone()).expect("Invalid output"),
            },
            success: run_output.status.success(),
            time: match run_output.status.success() {
                true => {
                    let re = Regex::new(r"\((\d+\.\d+) sec real").unwrap();
                    if let Some(caps) =
                        re.captures(std::str::from_utf8(&run_output.stderr).expect("failed"))
                    {
                        caps[1].parse().unwrap_or(0.0)
                    } else {
                        0.0
                    }
                }
                false => 0.0,
            },
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
