use actix_web::{post, web, Responder};
use hodu_core::{mark, MarkParams};

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

    let output = mark(MarkParams {
        language: match submission.language.as_str() {
            "c" => &hodu_core::Language::C,
            "c++" => &hodu_core::Language::Cpp,
            "java" => &hodu_core::Language::Java,
            "python" => &hodu_core::Language::Python,
            "javascript" => &hodu_core::Language::JavaScript,
            _ => panic!("Invalid language"),
        },
        code: &submission.code,
        expected_stdout: "",
        stdin: "",
        memory_limit: 4096000,
        time_limit: 10,
    })
    .await;

    Result::Ok(web::Json(serde_json::json!({
        "stdout": output.stdout,
        "stderr": output.stderr,
    })))
}
