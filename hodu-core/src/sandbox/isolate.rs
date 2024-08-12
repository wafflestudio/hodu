use rand::Rng;
use std::{path::PathBuf, str::FromStr};
use tokio::process::Command;

use crate::error::CoreError;

use super::{
    Sandbox, SandboxCommand, SandboxExecuteOptions, SandboxResult, SandboxResultStatus,
    SandboxSpecification,
};

pub struct IsolateSandbox {
    box_id: i32,
    path: PathBuf,
    memory_limit: u32,
    time_limit: f64,
}

impl Sandbox for IsolateSandbox {
    async fn create(environment: SandboxSpecification) -> Self {
        let box_id = rand::thread_rng().gen_range(0..1000);
        let init_output = Command::new("isolate")
            .arg(format!("--box-id={}", box_id))
            .arg("--cg")
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
    ) -> Result<SandboxResult, CoreError> {
        let (output, time, memory, status, is_internal_error) = match options {
            SandboxExecuteOptions::Sandboxed { stdin } => {
                let home_dir = self.path.to_str().unwrap().trim();

                std::fs::write(format!("{}/stdin.txt", home_dir), stdin)
                    .expect("Failed to write file");

                let result = Command::new("isolate")
                    .arg("--cg")
                    .arg(format!("--box-id={}", self.box_id))
                    .arg(format!("--processes={}", 128))
                    .arg(format!("--time={}", self.time_limit))
                    .arg(format!("--wall-time={}", 100))
                    .arg(format!("--cg-mem={}", self.memory_limit))
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

                let meta_time: f64 = parse_meta_file(&meta_content, "time", 0.0);
                let meta_cg_mem: u32 = parse_meta_file(&meta_content, "cg-mem", 0);
                let meta_cg_oom_killed: u32 = parse_meta_file(&meta_content, "cg-oom-killed", 0);
                let meta_status: String =
                    parse_meta_file(&meta_content, "status", "OK".to_string());

                std::fs::remove_file(format!("{}/stdin.txt", home_dir))
                    .expect("Failed to remove file");
                std::fs::remove_file(format!("{}/meta.txt", home_dir))
                    .expect("Failed to remove file");

                let mut is_internal_error = false;

                let status = if meta_cg_oom_killed == 1 {
                    SandboxResultStatus::MemoryLimitExceeded
                } else if meta_status == "RE" || meta_status == "SG" {
                    SandboxResultStatus::RuntimeError
                } else if meta_status == "TO" {
                    SandboxResultStatus::TimeLimitExceeded
                } else if meta_status == "XX" {
                    is_internal_error = true;
                    SandboxResultStatus::RuntimeError
                } else if !result.status.success() {
                    SandboxResultStatus::RuntimeError
                } else {
                    SandboxResultStatus::Success
                };

                (result, meta_time, meta_cg_mem, status, is_internal_error)
            }
            SandboxExecuteOptions::Unsandboxed => {
                let result = Command::new(command.binary)
                    .args(&command.args)
                    .current_dir(&self.path)
                    .output()
                    .await
                    .expect("Failed to execute");

                let status = match &result.status.success() {
                    true => SandboxResultStatus::Success,
                    false => SandboxResultStatus::RuntimeError,
                };

                (result, 0.0, 0, status, false)
            }
        };

        if is_internal_error {
            return Result::Err(CoreError::InternalError("Isolate Error".to_string()));
        }

        Result::Ok(SandboxResult {
            stdout: match output.status.success() {
                true => String::from_utf8(output.stdout.clone()).expect("Invalid output"),
                false => String::new(),
            },
            stderr: match output.status.success() {
                true => String::new(),
                false => String::from_utf8(output.stderr.clone()).expect("Invalid output"),
            },
            status,
            time,
            memory,
        })
    }

    async fn destroy(&self) {
        Command::new("isolate")
            .arg("--cg")
            .arg(format!("--box-id={}", self.box_id))
            .arg("--cleanup")
            .output()
            .await
            .expect("Failed to cleanup box");
    }
}

fn parse_meta_file<S: FromStr>(content: &str, key: &str, default: S) -> S {
    content
        .lines()
        .find(|line| line.starts_with(key))
        .and_then(|line| line.split(':').nth(1))
        .map(|value| value.trim().parse::<S>().ok())
        .flatten()
        .unwrap_or(default)
}
