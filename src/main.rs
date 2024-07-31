use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod languages;
use languages::{c::run_c_code, java::run_java_code};
use uuid::Uuid;

#[derive(Deserialize)]
enum Language {
    C,
    JAVA,
}

#[derive(Deserialize)]
struct CodeSubmission {
    language: Language,
    code: String,
}

#[derive(Serialize)]
pub struct ExecutionResult {
    stdout: String,
    stderr: String,
    success: bool,
}

#[get("/ping")]
async fn ping() -> impl Responder {
    "pong"
}

#[post("/submit")]
async fn submit_code(submission: web::Json<CodeSubmission>) -> impl Responder {
    let random_string: String = Uuid::new_v4().to_string();
    let temp_dir = format!("./.temp/{}", random_string);
    let temp_dir = std::path::Path::new(&temp_dir);
    std::fs::create_dir_all(temp_dir).unwrap();

    let output = match submission.language {
        Language::C => run_c_code(&submission.code, temp_dir),
        Language::JAVA => run_java_code(&submission.code, temp_dir),
    };

    std::fs::remove_dir_all(temp_dir).unwrap();

    web::Json(output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./.temp").unwrap();
    HttpServer::new(|| App::new().service(ping).service(submit_code))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
