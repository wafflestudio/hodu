use crate::error::CoreError;

pub mod isolate;
pub mod ulimit;

pub struct SandboxCommand<'a> {
    pub binary: &'a str,
    pub args: Vec<&'a str>,
}

pub struct SandboxResult {
    pub stdout: String,
    pub stderr: String,
    pub time: f64,
    pub memory: u32,
    pub status: SandboxResultStatus,
}

#[derive(PartialEq)]
pub enum SandboxResultStatus {
    Success,
    RuntimeError,
    TimeLimitExceeded,
    MemoryLimitExceeded,
}

pub struct SandboxSpecification {
    pub memory_limit: u32,
    pub time_limit: f64,
}

pub enum SandboxExecuteOptions<'a> {
    Sandboxed { stdin: &'a str },
    Unsandboxed,
}

pub trait Sandbox {
    async fn create(environment: SandboxSpecification) -> Self;
    async fn add_file(&self, filename: &str, content: &str);
    async fn execute(
        &self,
        command: &SandboxCommand,
        options: &SandboxExecuteOptions,
    ) -> Result<SandboxResult, CoreError>;
    async fn destroy(&self);
}
