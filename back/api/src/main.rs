mod config;
mod handlers;

//use crate::models:Status;  // no metido, el profesor lo metió
use actix_web::{web, App, HttpServer};

use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;

#[actix_web::main]
async fn main() -> io::Result<()> {
  //cargo la configuracion del entorno
    dotenv().ok();    
    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
  println!("Server at http://{}:{}/", config.server.host, config.server.port);  
  HttpServer::new(move || {
    App::new()    
        .data(pool.clone())        
        // App:route => routes para manejar rutas manualmente
        .route("/", web::get().to(hello))
        .route("/hey_1", web::get().to(status_1))
        .route("/hey_2", web::get().to(status_2))        
        .route("/hey_3", web::get().to(status_3))
    })        
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

