use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::{IpAddr};
use std::time::Duration;
use std::time::SystemTime;

#[derive(Deserialize, Clone, Serialize)]
pub struct Server {
    ip_address: IpAddr,
    port: u32,
    label: String,
    last_heartbeat: Option<SystemTime>
}

impl Server {
    pub fn key(&self) -> String {
       self.ip_address.to_string()
    }

    pub fn add_heartbeat(&mut self) -> () {
        self.last_heartbeat = Some(SystemTime::now());
    }
 }

pub fn get_availble_services() -> Vec<Server> {
    let mut availble_backends: &mut Vec<Server> = &mut Vec::new();
    // for server in pool.servers {
    //     if let Health::OK(true) = server.status {
    //         availble_backends.push(server)
    //     }
    // }
    
    availble_backends.to_vec()
} 