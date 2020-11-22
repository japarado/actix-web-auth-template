use crate::schema::*;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_deserializing)]
    pub password: String,
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    email: String,
    password: String,
}

impl From<web::Json<User>> for User {
    fn from(user: web::Json<User>) -> Self {
        Self {
            id: user.id.to_owned(),
            email: user.email.to_owned(),
            password: user.password.to_owned(),
        }
    }
}
