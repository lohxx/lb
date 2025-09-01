use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::any::Any;

use crate::services::storage::Storage;

mod discovery;
mod health_checks;
mod services;


#[actix_web::post("/heartbeat")]
async fn heartbeat(server: web::Json<discovery::Server>) -> impl Responder {
   let store = services::storage::storage_strategy();
   store.update(server);
   HttpResponse::Ok().body("")
}

#[actix_web::post("/register")]
async fn register(server: web::Json<discovery::Server>) -> impl Responder {
    let store = services::storage::storage_strategy();
    store.save(server);
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
