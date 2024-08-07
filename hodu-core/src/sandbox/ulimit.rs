use rand::Rng;
use std::path::{Path, PathBuf};
use std::process::Output;
use tokio::process::Command;

use super::{Sandbox, SandboxCommand, SandboxResult, SandboxSpecification};

pub struct UlimitSandbox {
    user_name: String,
    uid: u32,
    memory_limit: u32,
    time_limit: u32,
}

impl UlimitSandbox {
    fn create_random_user_name() -> String {
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>()
    }
}

impl Sandbox for UlimitSandbox {
    async fn create(environment: SandboxSpecification) -> Self {
        let user_name = Self::create_random_user_name();
        let uid = UlimitSandbox::add_user(&user_name).await;

        UlimitSandbox {
            user_name: user_name,
            uid: uid,
            memory_limit: environment.memory_limit,
            time_limit: environment.time_limit,
        }
    }

    async fn add_file(&self, filename: &str, content: &str) {
        let home_dir = self.home_dir();
        let source_path = format!("{}/{}", home_dir, filename);

        if PathBuf::from(&home_dir).exists() {
            Command::new("rm")
                .arg("-rf")
                .arg(&home_dir)
                .output()
                .await
                .expect("Failed to delete home directory");
        }

        Command::new("mkdir")
            .arg("-p")
            .arg(&home_dir)
            .output()
            .await
            .expect("Failed to create home directory");

        Command::new("sh")
            .arg("-c")
            .arg(format!("echo '{}' > {}", content, source_path))
            .output()
            .await
            .expect("Failed to write file");
    }

    async fn execute(&self, command: SandboxCommand<'_>, sandboxed: bool) -> SandboxResult {
        let run_output: Output = Command::new("bash")
            .uid(self.uid)
            .arg("-c")
            .arg(format!(
                "{} -t {} -m {}; cd {}; {} -v {} {}",
                UlimitSandbox::ulimit_command(),
                self.time_limit,
                self.memory_limit,
                self.home_dir(),
                UlimitSandbox::time_command(),
                command.binary,
                command.args.join(" ")
            ))
            .output()
            .await
            .expect("Failed to execute");

        SandboxResult {
            stdout: String::from_utf8(run_output.stdout).expect("Invalid output"),
            stderr: String::from_utf8(run_output.stderr).expect("Invalid output"),
            time: 0.0,
            success: run_output.status.success(),
        }
    }

    async fn destroy(&self) {
        self.delete_user().await;

        Command::new("rm")
        .arg("-rf")
        .arg(&self.home_dir())
        .output()
        .await
        .expect("Failed to delete home directory");
    }
}

trait UlimitCommands {
    fn ulimit_command() -> String;
    fn time_command() -> String;
    async fn add_user(user_name: &str) -> u32;
    async fn delete_user(&self);
    fn home_dir(&self) -> String;
}

#[cfg(target_os = "macos")]
impl UlimitCommands for UlimitSandbox {
    fn ulimit_command() -> String {
        "ulimit".to_string()
    }

    fn time_command() -> String {
        "gtime".to_string()
    }

    async fn add_user(user_name: &str) -> u32 {
        Command::new("dscl")
            .arg(".")
            .arg("-create")
            .arg(format!("/Users/{}", user_name))
            .output()
            .await
            .expect("Failed to create user");

        get_uid(user_name).await
    }

    async fn delete_user(&self) {
        Command::new("dscl")
            .arg(".")
            .arg("-delete")
            .arg(format!("/Users/{}", self.user_name))
            .output()
            .await
            .expect("Failed to delete user");
    }

    fn home_dir(&self) -> String {
        format!("/Users/{}", self.user_name)
    }
}

#[cfg(target_os = "linux")]
impl UlimitCommands for UlimitSandbox {
    fn ulimit_command() -> String {
        "ulimit".to_string()
    }

    fn time_command() -> String {
        "/usr/bin/time".to_string()
    }

    async fn add_user(user_name: &str) -> u32 {
        Command::new("useradd")
            .arg(user_name)
            .output()
            .await
            .expect("Failed to create user");

        get_uid(user_name).await
    }

    async fn delete_user(&self) {
        Command::new("userdel")
            .arg(&self.user_name)
            .output()
            .await
            .expect("Failed to delete user");

        let home_dir = format!("/home/{}", self.user_name);
        Command::new("rm")
            .arg("-rf")
            .arg(&home_dir)
            .output()
            .await
            .expect("Failed to delete home directory");
    }

    fn home_dir(&self) -> String {
        format!("/home/{}", self.user_name)
    }
}

async fn get_uid(user_name: &str) -> u32 {
    let output = Command::new("id")
        .arg("-u")
        .arg(user_name)
        .output()
        .await
        .expect("Failed to get uid");

    String::from_utf8(output.stdout)
        .expect("Invalid output")
        .trim()
        .parse()
        .expect("Failed to parse uid")
}
