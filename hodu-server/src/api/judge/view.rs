use actix_web::{post, web, Responder};
use hodu_core::mark_code;

use crate::api::judge::schema::CodeSubmission;

#[post("/submit")]
async fn submit_code(submission: web::Json<CodeSubmission>) -> impl Responder {
    let output = mark_code(&submission.language, submission.code.clone()).await;

    web::Json(output)
}
