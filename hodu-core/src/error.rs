#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error("Internale Error: {0}")]
    InternalError(String),
}
