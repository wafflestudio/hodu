use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;
use uuid::Uuid;

extern crate judge_core;
use judge_core::{run_c_code, run_java_code, Language};

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
    let home_dir = std::env::var("HOME").expect("cannot find home directory");
    let random_string: String = Uuid::new_v4().to_string();
    let temp_dir = format!("{}/.waffle-judge/temp/{}", home_dir, random_string);
    let temp_dir = std::path::Path::new(&temp_dir);

    std::fs::create_dir_all(&temp_dir).unwrap();

    let output = match submission.language {
        Language::C => run_c_code(&submission.code, &temp_dir),
        Language::JAVA => run_java_code(&submission.code, &temp_dir),
    };

    std::fs::remove_dir_all(temp_dir).unwrap();

    web::Json(output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let home_dir = std::env::var("HOME").expect("cannot find home directory");
    let waffle_judge_dir = format!("{}/.waffle-judge/temp", home_dir);
    std::fs::create_dir_all(&waffle_judge_dir).unwrap();
    HttpServer::new(|| App::new().service(ping).service(submit_code))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
