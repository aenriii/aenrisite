#![feature(test)]
#![feature(async_closure)]
mod html;
mod routes;

#[cfg(test)]
mod tests;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
pub(crate) use html::{components, layouts, styles};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let style = |src: &'static str| {
        move || async move { HttpResponse::Ok().content_type("text/css").body(src) }
    };
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(routes::index)
            .service(web::resource("/base.css").to(style(styles::BASE)))
            .service(web::resource("/main_layout.css").to(style(styles::MAIN_LAYOUT)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
