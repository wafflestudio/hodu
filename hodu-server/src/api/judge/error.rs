use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(thiserror::Error, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "detail")]
pub enum JudgeError {
    #[error("PayloadParseError: {0}")]
    PayloadParseError(
        #[source]
        #[serde(skip)]
        actix_web::Error,
    ),
    #[error("HoduCoreError")]
    HoduCoreError,
}

impl ResponseError for JudgeError {
    fn status_code(&self) -> StatusCode {
        match self {
            JudgeError::PayloadParseError(_) => StatusCode::BAD_REQUEST,
            JudgeError::HoduCoreError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = self;
        HttpResponse::build(status_code).json(error_response)
    }
}
