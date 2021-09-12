//endpoints

//doc diesel
//https://docs.diesel.rs/master/diesel/index.html

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


// Ruta inicial: /
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("api en rust")
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

// Handler for POST /users
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    
    //imprime usuario creado
    println!("{:?}", item);  
    
    Ok(web::block(move || add_single_user(db, item))    
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn add_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<Users, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        name: &item.name,
        adress: &item.adress,
        telephone: &item.telephone,
        email: &item.email,
        password: &item.password,
        comments: &item.comments,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(tbusers).values(&new_user).get_result(&conn)?;
    Ok(res)
}

// Handler for DELETE /users/{id}
pub async fn delete_user(
    db: web::Data<Pool>,
    //usuario de la url
    user_id: web::Path<i32>,    
) -> Result<HttpResponse, Error> {
   
    //imprime usuario a eliminar
    println!("{:?}", user_id);    
   
    Ok(
        web::block(move || delete_single_user(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,     
    )    
}

fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(tbusers.find(user_id)).execute(&conn)?;
    Ok(count)
}
