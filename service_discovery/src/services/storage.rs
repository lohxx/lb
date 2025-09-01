use std::any::Any;

pub mod redis;
pub mod sqlite;

pub trait Storage {
    fn save<T>(&self, entity: T) -> Option<T>;
    fn get<T>(&self, identifier: T) -> Option<T>;
    fn update<T>(&self, entity: T) -> Option<T>;
}

pub fn storage_strategy() -> impl Storage {
    redis::RedisStore::new()
}