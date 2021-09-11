//creamos este fichero para organizar.
//Metemos aqui las rutas

use actix_web::{HttpResponse, Responder};

//endpoint => http://127.0.0.1:8080/
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("api en rust")
}

//HttpResponse
//endpoint => http://127.0.0.1:8080/hey_1
pub async fn status_1() -> impl Responder {
    //esto es lo que se devuelve en el navegador => status_1
    HttpResponse::Ok().body("status_11")
}

// devuelve un json
//endpoint => http://127.0.0.1:8080/hey_2
pub async fn status_2() -> impl Responder {
    "{\"nombre\": \"javier\"}"    // esto genera un objeto =>  {"nombre": "javier"}
}

//endpoint => http://127.0.0.1:8080/hey_3
// devuelve un json, otra forma
pub async fn status_3() -> impl Responder {
    //"hola"
    HttpResponse::Ok()
        //.content_type("application/xml")
        .content_type("application/json")
        //.content_type("text/html")
        .body("{\"nombre\": \"javier\"}" )
        //.json(Status { status_3: "UP".to_string() })
}
