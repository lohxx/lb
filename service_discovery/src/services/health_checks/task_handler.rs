use crate::discovery;
use crate::types;


use std::time::SystemTime;

use crate::services::storage::{storage_strategy, Storage};
use crate::types::{Health, Server};

trait ServicesStatus {
    fn get_registered_services(&self) -> Vec<Server>;
}

// Salvar os serviços registrados e rodar health_checks, o cliente que vai determinar a peridiocidade do health check?
// Se sim, vamos precisar de um loop que verifica as chamadas e marca o serviço como saudavel ou não, isso em conjunto
// com a taxa de resposta do serviço através da repassagem das chamadas para os serviços.

pub fn check_servers_health_state() {
    loop {
        let mut _servers: Vec<types::Server> = discovery::get_availble_services();
        // Dispara a verificação entre varias threads


    }
}