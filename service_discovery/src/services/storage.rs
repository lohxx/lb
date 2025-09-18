pub mod redis;
pub mod sqlite;

use crate::types::Server;

use std::{any::Any, error::Error};

pub trait Storage: Send + Sync {
    //fn save(&mut self, identifier: &str, value: Server) -> Result<(), Box<dyn Error>>;
    fn update(&mut self, identifier: &str, value: Server) -> Result<(), Box<dyn Error>>;
    fn get(&mut self, identifier: &str) -> Result<Server, Box<dyn Error>>;
}

pub fn storage_strategy() -> redis::RedisStore {
    redis::RedisStore::new()
}