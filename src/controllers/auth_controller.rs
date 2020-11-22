use crate::models::user;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[post("/register")]
pub async fn register(payload: web::Json<user::NewUser>) -> impl Responder {
    HttpResponse::Ok().json("register route")
}
