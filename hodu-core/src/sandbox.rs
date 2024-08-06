pub mod isolate;

pub struct SandboxCommand<'a> {
    pub binary: &'a str,
    pub args: Vec<&'a str>,
}

pub struct SandboxResult {
    pub stdout: String,
    pub stderr: String,
    pub time: f64,
    pub success: bool,
}

pub struct SandboxSpecification {
    pub memory_limit: u32,
    pub time_limit: u32,
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
    ) -> SandboxResult;
    async fn destroy(&self);
}
