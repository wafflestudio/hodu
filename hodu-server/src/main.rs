use std::sync::atomic::AtomicU32;

use actix_web::{web, App, HttpServer};

mod api;

pub struct MarkCounter {
    count: AtomicU32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let counter = web::Data::new(MarkCounter {
        count: AtomicU32::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(api::v1_router())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
