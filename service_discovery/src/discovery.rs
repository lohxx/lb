use serde::Deserialize;
use std::net::{IpAddr};

#[derive(Deserialize, Clone)]
pub struct Server {
    ip_address: IpAddr,
    port: u32,
    label: String,
    //last_heartbeat: 
}

#[derive(Deserialize, Clone)]
pub enum Health {
    OK(bool),
}

pub struct BackEndPool {
    servers: Vec<Server>
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