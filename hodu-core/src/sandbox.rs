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
    pub success: bool,
}

pub struct SandboxSpecification {
    pub memory_limit: u32,
    pub time_limit: u32,
}

pub trait Sandbox {
    async fn create(environment: SandboxSpecification) -> Self;
    async fn add_file(&self, filename: &str, content: &str);
    async fn execute(&self, command: SandboxCommand, sandboxed: bool) -> SandboxResult;
    async fn destroy(&self);
}
