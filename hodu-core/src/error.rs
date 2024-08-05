use crate::sandbox::isolate;

#[derive(thiserror::Error, Debug)]
pub enum HoduCoreError {
    #[error("IsolateError: {0}")]
    IsolateError(#[source] isolate::Error),
}
