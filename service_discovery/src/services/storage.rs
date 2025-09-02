pub mod redis;
pub mod sqlite;

use std::error::Error;


pub trait Storage {
    fn save(&mut self, identifier: &str, value: &str) -> Result<(), Box<dyn Error>>;
    fn update(&mut self, identifier: &str, value: &str) -> Result<(), Box<dyn Error>>;
    fn get(&mut self, identifier: &str) -> Result<Option<String>, Box<dyn Error>>;
}

pub fn storage_strategy() -> impl Storage {
    redis::RedisStore::new()
}