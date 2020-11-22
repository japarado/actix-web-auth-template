#[macro_use]
extern crate diesel;

#[macro_use]
extern crate failure;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

// HTTP
mod controllers;
mod middleware;
mod routes;

// Layers
mod services;
mod repositories;

// Database
mod database;
mod models;
mod schema;

// Misc
mod errors;
mod utils;

#[derive(Debug, Clone)]
pub struct AppData {
    conn_pool: database::Pool
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_data = AppData {
        conn_pool: database::create_pool()
    };

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(app_data.clone())
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
