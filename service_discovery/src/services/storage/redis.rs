use redis::{Commands, ToRedisArgs};
use redis::{Client, Connection};
use serde_json;

use std::error::Error;
use crate::services::storage::{Storage};
use crate::types::Server;

pub enum RedisDS {
    Hash,
    Set,
    List
}

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
}

impl Storage for RedisStore {
    fn get(&mut self, identifier: &str) -> Result<Server, Box<dyn Error>> {
        let result: Option<String> = Commands::get(&mut self.connection, identifier)?;
        let deserialized_data: Server = serde_json::from_str(result.unwrap().as_str()).unwrap();
        Ok(deserialized_data.clone())
    }

    fn update(&mut self, identifier: &str, value: Server) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn get_servers_checked_from_last_seconds(&mut self, last_time_checked: u64) -> Result<Vec<Server>, Box<dyn Error>> {
        Ok(vec![])
    }

    fn save(&mut self, value: Server) -> Result<(), Box<dyn Error>> {
        let key = value.key();
        let serialized_value = serde_json::to_string(&value).unwrap();
        self.connection.set(key, serialized_value)

        Ok()
    }

}

