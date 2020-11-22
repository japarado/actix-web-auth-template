#[macro_use]
extern crate diesel;

#[macro_use]
extern crate failure;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod controllers;
mod middleware;
mod models;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .service(index)
            .default_service(web::route().to(fallback))
            .configure(routes::config)
            .wrap(middleware::auth::Auth)
            .wrap(Cors::new().finish())
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        let app_port = env::var("APP_URL").unwrap_or("localhost:8000".to_string());
        server.bind(app_port)?
    };

    server.run().await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Auth Template API Root")
}

async fn fallback(_req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().json("Not found")
}
