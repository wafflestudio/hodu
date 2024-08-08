use std::panic::AssertUnwindSafe;

use actix_web::{post, web, Responder};
use hodu_core::{mark, MarkParams};

use futures::FutureExt;

use crate::api::judge::{
    error::JudgeError,
    schema::{CodeSubmission, MarkResponse},
};

#[post("/submit")]
async fn submit_code(
    submission: Result<web::Json<CodeSubmission>, actix_web::Error>,
) -> Result<impl Responder, JudgeError> {
    let submission = submission.map_err(|e| JudgeError::PayloadParseError(e))?;
    tracing::info!(
        "Received code submission: {} / {:?}",
        submission.id,
        submission.language
    );

    let output = AssertUnwindSafe(mark(MarkParams {
        language: &submission.language.clone().into(),
        code: &submission.code,
        expected_stdout: &submission.expected_stdout,
        stdin: &submission.stdin,
        memory_limit: submission.memory_limit,
        time_limit: submission.time_limit,
    }))
    .catch_unwind()
    .await
    .map_err(|_| JudgeError::HoduCoreError)?;

    Ok(web::Json(
        serde_json::to_value(MarkResponse::new(&output, &submission.fields)).unwrap(),
    ))
}
