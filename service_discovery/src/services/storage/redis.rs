use redis::Commands;
use redis::{Client, Connection};
use redis::RedisResult;
use redis::ToRedisArgs;

use crate::services::storage::{Storage};

pub struct RedisStore {
    client: Client,
    connection: Option<Connection>
}

impl RedisStore {
    pub fn new() -> Self {
        let client = Client::open("redis://127.0.0.1").unwrap();

        RedisStore { 
            // deixar variavel conforme as configurações da aplicação.
            client: client,
            connection: None
        }
    }
}

impl Storage for RedisStore {
    fn get<T>(&self, identifier: T) -> Option<T> {
        let connection: Connection = self.client.get_connection().unwrap();
        None
    }

    fn save<T>(&self, entity: T) -> Option<T> {
        None
    }

    fn update<T>(&self, entity: T) -> Option<T> {
        None
    }
}