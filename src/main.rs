#![feature(async_closure)]
mod html;
mod routes;
pub(crate) mod collection;
use actix_web::{middleware, web::resource, App, HttpResponse, HttpServer};
pub(crate) use html::{components, frames, styles};
use marker::Page;
use marker_macro::marker;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    log::info!("Hello.");
    let style = |src: &'static str| {
        move || async move { HttpResponse::Ok().content_type("text/css").body(src) }
    };



    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(routes::homepage)
            .service(resource("/assets/base.css").to(style(styles::BASE)))
            .service(resource("/assets/home.css").to(style(styles::HOME)))
            .service(actix_files::Files::new("/assets", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
