use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::error::Error;
use std::ops::Deref;
use std::sync::Mutex;
use std::sync::Arc;


use crate::services::storage::{Storage};
use crate::services::storage::redis::RedisDS;

mod discovery;
mod services;
mod types;


type AppControler<'a> = std::sync::MutexGuard<'a, CrudController>;

struct AppState {
    controller: CrudController
}

struct CrudController {
    store: Arc<Mutex<dyn Storage>>,
}

impl CrudController {
    pub fn new(store: Arc<Mutex<dyn Storage>>) -> Self {
        Self { store: store.lock().unwrap() }
    }

    pub fn save(&mut self, server: types::Server) -> Result<(), ()> {
        let mut res = &self.store.save(server);

        Ok(())
    }

    pub fn read(&mut self, id: String) -> Result<types::Server, ()> {
        let result_ref: &Result<types::Server, Box<dyn Error>> = &self.store.get(&id);
        if result_ref.is_ok() {
            return Ok(result_ref.unwrap());
        }

        Ok(())
    }
}

#[actix_web::post("/service_discovery/server/register")]
async fn register(server: web::Json<types::Server>, data: web::Data<AppState>) -> impl Responder {
    let mut controller: AppControler  = data.controller.lock().unwrap();
    let service = controller.save(server.to_owned());
    if service.is_ok() {
        return HttpResponse::Created().body("")
    }

    return HttpResponse::InternalServerError()
        .body("Unexpected error while trying to process the request")
}

#[actix_web::get("/service_discovery/server/{server_id}")]
async fn get(server_id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut controller: AppControler = data.controller.lock().unwrap();
    HttpResponse::Ok()
}

#[actix_web::delete("/service_discovery/server/{server_id}")]
async fn delete(server_id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut controller: AppControler = data.controller.lock().unwrap();
    HttpResponse::Ok()
}

#[actix_web::patch("/service_discovery/server/{server_id}")]
async fn update(server_id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut controller: AppControler = data.controller.lock().unwrap();
    HttpResponse::Ok()
}


#[actix_web::main]
async fn main() ->std::io::Result<()> {
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        controller: CrudController::new(
            Arc::new(Mutex::new(services::storage::storage_strategy())),
        )
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(register)
    })
    .bind(("0.0.0.0", 8006))?
    .run()
    .await
}
