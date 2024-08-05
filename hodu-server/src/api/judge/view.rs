use actix_web::{post, web, Responder};
use hodu_core::mark_code;

use crate::api::{error::HoduError, judge::schema::CodeSubmission};

#[post("/submit")]
async fn submit_code(
    submission: Result<web::Json<CodeSubmission>, actix_web::Error>,
) -> Result<impl Responder, HoduError> {
    let submission = submission.map_err(|e| HoduError::PayloadParseError(e))?;
    tracing::info!(
        "Received code submission: {} / {:?}",
        submission.id,
        submission.language
    );

    let output = mark_code(&submission.language, submission.code.clone()).await.map_err(HoduError::CodeExecutionError)?;

    Ok(web::Json(output))
}
