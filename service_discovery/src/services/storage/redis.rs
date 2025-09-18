use redis::{Commands, ToRedisArgs};
use redis::{Client, Connection};
use serde_json;

use std::error::Error;
use crate::services::storage::{Storage};
use crate::types::Server;

pub struct RedisStore {
    connection: Connection
}

impl RedisStore {
    pub fn new() -> Self {
        let client: Client = Client::open("redis://0.0.0.0").unwrap();

        RedisStore { 
            connection: client.get_connection().unwrap()
        }
    }

    pub fn save(&mut self, key: &str, value: Server, save_as: Option<RedisDS>) -> Result<(), Box<dyn Error>> {
        let serialized_value = serde_json::to_string(&value).unwrap();
        match save_as {
            Some(RedisDS::Hash) => self.connection.set(key, serialized_value)?,
            Some(RedisDS::Set) => self.connection.sadd(key, serialized_value)?,
            _ => Commands::set(&mut self.connection, key, serialized_value)?
        };

        Ok(())
    }
}

impl Storage for RedisStore {
    fn get(&mut self, identifier: &str) -> Result<Server, Box<dyn Error>> {
        let result: Option<String> = Commands::get(&mut self.connection, identifier)?;
        let deserialized_data: Server = serde_json::from_str(result.unwrap().as_str()).unwrap();
        Ok(deserialized_data)
    }

    fn update(&mut self, identifier: &str, value: Server) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

}

pub enum RedisDS {
    Hash,
    Set,
    List
}