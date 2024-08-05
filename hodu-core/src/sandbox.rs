pub mod isolate;

pub struct ExecutionCommand<'a> {
    pub binary: &'a str,
    pub args: Vec<&'a str>,
}

pub struct ExecutionResult {
    pub output: String,
    pub success: bool,
}

pub struct SandboxEnvironment {
    pub memory_limit: u32,
    pub time_limit: u32,
}

pub trait Sandbox {
    async fn create(environment: SandboxEnvironment) -> Self;
    async fn add_file(&self, filename: &str, content: &str);
    async fn execute(&self, command: ExecutionCommand, sandboxed: bool) -> ExecutionResult;
    async fn destroy(&self);
}
