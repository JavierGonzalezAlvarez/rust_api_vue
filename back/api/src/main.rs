// declarar modulos
mod handlers;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

//cors
use actix_cors::Cors;
use actix_web::http::header;

// dependencies here
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// type declarations here
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
         .build(manager)
        .expect("Failed to create pool.");   // en vez de poner ? coloco expect() para controlar el error    

    println!("Server at http://127.0.0.1:8080");  

    // Start http server
    HttpServer::new(move || {
        //Auth0
        //let auth = HttpAuthentication::bearer(validator);
        
        let _cors = Cors::default()        
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .allowed_header(header::ACCESS_CONTROL_ALLOW_ORIGIN)
        //.allowed_origin("http://127.0.0.1:8081")        
        .allowed_origin("*")    
        .allowed_methods(vec!["GET", "POST"])        
        .max_age(3600);                

        App::new()
        //Auth0
            //.wrap(auth)
            .data(pool.clone())
            
            //endpoint => http://127.0.0.1:8080/
            .route("/", web::get().to(handlers::hello))    
            
            //endpoint => http://127.0.0.1:8080/users
            .route("/users", web::get().to(handlers::get_users))
            
            //endpoint => 
            //.route("/userid/{id}", web::get().to(handlers::get_user_by_id))
            
            //endpoint => http://127.0.0.1:8080/adduser
            .route("/adduser", web::post().to(handlers::add_user))

            //endpoint => http://127.0.0.1:8080/deleteuser/id
            .route("/deleteuser/{id}", web::delete().to(handlers::delete_user))
    })
    .bind("127.0.0.1:8080")?            
    .run()
    .await
}




