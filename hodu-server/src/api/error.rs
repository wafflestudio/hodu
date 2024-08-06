use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum HoduError {
    #[error("PayloadParseError: {0}")]
    PayloadParseError(#[source] actix_web::Error),
    #[error("CodeExecutionError: {0}")]
    CodeExecutionError(#[source] hodu_core::HoduCoreError),
}

#[derive(Serialize, Debug)]
struct HoduErrorResponse {
    detail: String,
}

impl ResponseError for HoduError {
    fn status_code(&self) -> StatusCode {
        match self {
            HoduError::PayloadParseError(_) => StatusCode::BAD_REQUEST,
            HoduError::CodeExecutionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let detail = self.to_string();
        let error_response = HoduErrorResponse { detail };
        HttpResponse::build(status_code).json(error_response)
    }
}
