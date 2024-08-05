use actix_web::{dev::HttpServiceFactory, web};

mod judge;
mod ping;

pub fn v1_router() -> impl HttpServiceFactory {
    web::scope("/api/v1")
        .service(ping::view::ping)
        .service(judge::view::submit_code)
}
