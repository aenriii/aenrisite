#![feature(async_closure)]

mod web;

use actix_web::{middleware, App, HttpServer};
use web::Router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/assets", "./static"))
            .setup_router()

    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
