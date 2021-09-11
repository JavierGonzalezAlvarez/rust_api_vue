// src/models.rs

use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Users {
    pub id: i32,
    pub name: String,    
    pub adress: String,    
    pub telephone: String,    
    pub email: String,    
    pub password: String,    
    pub comments: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug, Queryable)]
#[table_name = "tbusers"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub adress: &'a str,    
    pub telephone: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub comments: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
