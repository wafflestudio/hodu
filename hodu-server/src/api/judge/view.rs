use std::{panic::AssertUnwindSafe, sync::atomic::Ordering, time::Duration};

use actix_web::{post, web, Responder};
use hodu_core::{mark, MarkParams};

use futures::FutureExt;

use crate::{
    api::judge::{
        error::JudgeError,
        schema::{CodeSubmission, MarkResponse},
    },
    MarkCounter,
};

const MAX_SUBMISSIONS: u32 = 20;

#[post("/submit")]
async fn submit_code(
    submission: Result<web::Json<CodeSubmission>, actix_web::Error>,
    counter: web::Data<MarkCounter>,
) -> Result<impl Responder, JudgeError> {
    let submission = submission.map_err(JudgeError::PayloadParseError)?;
    tracing::info!(
        "Received code submission: {} / {:?}",
        submission.id,
        submission.language
    );

    let mut current_count = counter.count.load(Ordering::SeqCst);
    while current_count >= MAX_SUBMISSIONS {
        tracing::info!("Max submissions reached, waiting for a slot...");
        tokio::time::sleep(Duration::from_secs(1)).await;
        current_count = counter.count.load(Ordering::SeqCst);
    }
    let previous_count = counter.count.fetch_add(1, Ordering::SeqCst);
    tracing::info!("Current count: {}", previous_count + 1);
    let output = AssertUnwindSafe(mark(MarkParams {
        language: &submission.language.clone().into(),
        code: &submission.code,
        compile_options: &submission
            .compile_options
            .as_ref()
            .map(|options| options.iter().map(String::as_str).collect()),
        expected_stdout: &submission.expected_stdout,
        stdin: &submission.stdin,
        memory_limit: submission.memory_limit,
        time_limit: submission.time_limit,
    }))
    .catch_unwind()
    .await
    .map_err(|_| {
        counter.count.fetch_sub(1, Ordering::SeqCst);
        JudgeError::HoduCoreError
    })?;
    counter.count.fetch_sub(1, Ordering::SeqCst);

    Ok(web::Json(
        serde_json::to_value(MarkResponse::new(&output, &submission.fields)).unwrap(),
    ))
}
