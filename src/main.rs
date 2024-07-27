use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod languages;
use languages::{c::run_c_code, java::run_java_code};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

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

#[post("/submit")]
async fn submit_code(submission: web::Json<CodeSubmission>) -> impl Responder {
    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
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
    HttpServer::new(|| App::new().service(submit_code))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
