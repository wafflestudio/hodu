use crate::sandbox::Sandbox;

pub mod c;
pub mod cpp;
pub mod java;
pub mod javascript;
pub mod python;

pub trait LanguageExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult;
}

pub struct ExecutionSuccessOutput {
    pub time: f64,
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
}
