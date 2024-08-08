use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum JudgeError {
    #[error("PayloadParseError: {0}")]
    PayloadParseError(#[source] actix_web::Error),
}

#[derive(Serialize, Debug)]
struct JudgeErrorResponse {
    detail: String,
}

impl ResponseError for JudgeError {
    fn status_code(&self) -> StatusCode {
        match self {
            JudgeError::PayloadParseError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let detail = self.to_string();
        let error_response = JudgeErrorResponse { detail };
        HttpResponse::build(status_code).json(error_response)
    }
}
