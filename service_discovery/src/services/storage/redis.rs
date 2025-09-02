use redis::Commands;
use redis::{Client, Connection};

use std::error::Error;


use crate::services::storage::{Storage};

pub struct RedisStore {
    connection: Connection
}

impl RedisStore {
    pub fn new() -> Self {
        let client: Client = Client::open("redis://127.0.0.1").unwrap();

        RedisStore { 
            connection: client.get_connection().unwrap()
        }
    }
}

impl Storage for RedisStore {
    fn get(&mut self, identifier: &str) -> Result<Option<String>, Box<dyn Error>> {
        let result: Option<String> = self.connection.get(identifier)?;
        Ok(result)
    }

    fn save(&mut self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let _: () = self.connection.set(key, value)?;
        Ok(())
    }

    fn update(&mut self, identifier: &str, value: &str) -> Result<(), Box<dyn Error>> {
        self.save(identifier, value);
        Ok(())
    }
}