use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;

extern crate hodu_core;
use hodu_core::{mark_code, Language};

#[derive(Deserialize)]
struct CodeSubmission {
    language: Language,
    code: String,
}

#[get("/ping")]
async fn ping() -> impl Responder {
    "pong"
}

#[post("/submit")]
async fn submit_code(submission: web::Json<CodeSubmission>) -> impl Responder {
    let output = mark_code(&submission.language, submission.code.clone()).await;

    web::Json(output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ping).service(submit_code))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
