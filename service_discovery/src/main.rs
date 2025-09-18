use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::sync::Mutex;
use std::sync::Arc;


use crate::services::storage::Storage;

mod discovery;
mod services;
mod types;


struct AppState {
    controller: Mutex<Controller>
}

struct Controller {
    store: Arc<dyn Storage>,
}

impl Controller {
    pub fn new(store: Arc<dyn Storage>) -> Self {
        Self { store: store }
    }

    pub fn save_entry(&mut self, server: types::Server) -> Result<(), ()> {
        Ok(())
    }
}

//

#[actix_web::post("/register")]
async fn register(server: web::Json<types::Server>, data: web::Data<AppState>) -> impl Responder {
    let mut controller: std::sync::MutexGuard<'_, Controller> = data.controller.lock().unwrap();
    let status = controller.save_entry(server.to_owned());
    HttpResponse::Ok().body("")
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        controller: Mutex::new(
            Controller::new(
                Arc::new(services::storage::storage_strategy()))),
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
