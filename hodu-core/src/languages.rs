use crate::{error::CoreError, sandbox::Sandbox};

pub mod c;
pub mod cpp;
pub mod java;
pub mod javascript;
pub mod python;

pub trait LanguageExecutor {
    async fn run(
        &self,
        params: &ExecutionParams,
        sandbox: &impl Sandbox,
    ) -> Result<ExecutionResult, CoreError>;
}

pub struct ExecutionParams<'a> {
    pub code: &'a str,
    pub stdin: &'a str,
    pub compile_options: &'a Option<Vec<&'a str>>,
}

pub struct ExecutionSuccessOutput {
    pub time: f64,
    pub memory: u32,
    pub stdout: String,
    pub stderr: String,
}

pub struct ExecutionErrorOutput {
    pub stdout: String,
    pub stderr: String,
}

pub enum ExecutionResult {
    Success(ExecutionSuccessOutput),
    CompileError(ExecutionErrorOutput),
    RuntimeError(ExecutionErrorOutput),
    MemoryLimitExceeded,
    TimeLimitExceeded,
}
