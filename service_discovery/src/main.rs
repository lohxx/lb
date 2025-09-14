use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde_json;
use std::any::Any;

use crate::services::storage::Storage;

mod discovery;
mod health_checks;
mod services;
mod types;


#[actix_web::post("/register")]
async fn register(server: web::Json<types::Server>) -> impl Responder {
    let mut store = services::storage::storage_strategy();
    let key = server.key();
    store.save(&key, server.to_owned(), None).unwrap();
    HttpResponse::Ok().body("")
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //.service(heartbeat)
            .service(register)
    })
    .bind(("0.0.0.0", 8006))?
    .run()
    .await
}
