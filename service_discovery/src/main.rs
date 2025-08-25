use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use servers::{Server}

#[actix_web::post("/heartbeat")]
async fn heartbeat(server: web::Json<Server>) -> impl Responder {
    HttpResponse::Ok().body("")
}

#[actix_web::post("/register")]
async fn register(server: web::Json<Server>) -> impl Responder {
    HttpResponse::Ok().body(server.ip_address.to_string())
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
