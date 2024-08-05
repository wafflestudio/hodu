use crate::sandbox::{ExecutionResult, Sandbox};

pub mod c;
pub mod cpp;
pub mod java;
pub mod javascript;
pub mod python;

pub trait LanguageExecutor {
    async fn run<S: Sandbox>(&self, code: &str, sandbox: &S) -> ExecutionResult;
}
