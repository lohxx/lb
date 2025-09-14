
use std::time::SystemTime;

use crate::services::storage::{storage_strategy, Storage};
use crate::types::{Health, Server};

trait ServicesStatus {
    fn get_availble_services(&self) -> Vec<Server>;
}

pub fn get_availble_services() -> Vec<Server> {
    let mut available: Vec<Server> = vec![];
    let mut strategy  = storage_strategy();

    available

} 