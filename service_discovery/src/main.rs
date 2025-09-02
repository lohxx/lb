use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde_json;
use std::any::Any;

use crate::services::storage::Storage;

mod discovery;
mod health_checks;
mod services;


#[actix_web::post("/heartbeat")]
async fn heartbeat(mut server: web::Json<discovery::Server>) -> impl Responder {
   let mut store = services::storage::storage_strategy();
   server.add_heartbeat();
   let key = server.key();
   let serialize = serde_json::to_string(&server).unwrap();
   store.update(&key, &serialize).unwrap();
   HttpResponse::Ok().body("")
}

#[actix_web::post("/register")]
async fn register(server: web::Json<discovery::Server>) -> impl Responder {
    let mut store = services::storage::storage_strategy();
    let key = server.key();
    let serialize = serde_json::to_string(&server).unwrap();
    store.save(&key, &serialize).unwrap();
    HttpResponse::Ok().body("")
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(heartbeat)
            .service(register)
    })
    .bind(("0.0.0.0", 8006))?
    .run()
    .await
}
