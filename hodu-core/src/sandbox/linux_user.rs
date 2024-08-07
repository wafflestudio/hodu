use rand::{distributions::Alphanumeric, Rng};
use std::path::PathBuf;

use tokio::process::Command;

use super::{Sandbox, SandboxCommand, SandboxExecuteOptions, SandboxResult, SandboxSpecification};

pub struct LinuxUserSandbox {
    user: String,
    memory_limit: u32,
    time_limit: u32,
}

impl Sandbox for LinuxUserSandbox {
    async fn create(environment: SandboxSpecification) -> Self {
        let user: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        Command::new("useradd")
            .arg("-m")
            .arg(&user)
            .output()
            .await
            .expect("Failed to create user");

        LinuxUserSandbox {
            user,
            memory_limit: environment.memory_limit,
            time_limit: environment.time_limit,
        }
    }

    async fn add_file(&self, filename: &str, content: &str) {
        let source_path = PathBuf::from(format!("/home/{}/{}", self.user, filename));
        std::fs::write(source_path, content).expect("Failed to write file");
    }

    async fn execute(
        &self,
        command: &SandboxCommand<'_>,
        options: &SandboxExecuteOptions<'_>,
    ) -> SandboxResult {
        let uid = String::from_utf8(
            Command::new("id")
                .arg("-u")
                .arg(&self.user)
                .output()
                .await
                .expect("Failed to get user id")
                .stdout,
        )
        .expect("Failed to convert uid to string")
        .trim()
        .parse::<u32>()
        .expect("Failed to parse uid");

        let (output, time) = match options {
            SandboxExecuteOptions::Sandboxed { stdin } => {
                std::fs::write(format!("/home/{}/input.txt", self.user), stdin)
                    .expect("Failed to write stdin");

                let start_time = std::time::Instant::now();

                let output = Command::new("bash")
                    .uid(uid)
                    .arg("-c")
                    .arg(format!(
                        "ulimit -t {} -m {}; cd {}; {} -v {} {} < {}",
                        self.time_limit,
                        self.memory_limit,
                        format!("/home/{}", self.user),
                        "/usr/bin/time",
                        command.binary,
                        command.args.join(" "),
                        format!("/home/{}/input.txt", self.user)
                    ))
                    .output()
                    .await
                    .expect("Failed to execute command");

                let time = start_time.elapsed().as_secs_f64();

                std::fs::remove_file(format!("/home/{}/input.txt", self.user))
                    .expect("Failed to remove input file");

                (output, time)
            }
            SandboxExecuteOptions::Unsandboxed => {
                let start_time = std::time::Instant::now();

                let output = Command::new(command.binary)
                    .args(&command.args)
                    .current_dir(PathBuf::from(format!("/home/{}", self.user)))
                    .output()
                    .await
                    .expect("Failed to execute command");

                let time = start_time.elapsed().as_secs_f64();

                (output, time)
            }
        };

        SandboxResult {
            stdout: match output.status.code() {
                Some(0) => {
                    String::from_utf8(output.stdout).expect("Failed to convert stdout to string")
                }
                _ => String::new(),
            },
            stderr: match output.status.code() {
                Some(0) => String::new(),
                _ => String::from_utf8(output.stderr).expect("Failed to convert stdout to string"),
            },
            success: match output.status.code() {
                Some(0) => true,
                _ => false,
            },
            time,
        }
    }

    async fn destroy(&self) {
        // TODO: implement
    }
}
