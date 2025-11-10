use actix_web::dev::Server;
use crate::services::storage::Storage;
use crate::types;

pub fn get_availble_services(repository: impl Storage) -> Vec<types::Server> {
    vec![]
}