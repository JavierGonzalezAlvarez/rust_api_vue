//endpoints

// dependencias
use super::models::{NewUser, Users};
use super::schema::tbusers::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub name: String,
    pub adress: String,
    pub telephone: String,
    pub email: String,
    pub password: String,
    pub comments: String,
}

//GET /users
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<Users>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = tbusers.load::<Users>(&conn)?;
    Ok(items)
}

// Ruta inicial: /
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("api en rust")
}

