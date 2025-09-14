use serde::{Deserialize, Serialize};
use std::net::{IpAddr};
use std::time::SystemTime;

#[derive(Deserialize, Clone, Serialize)]
pub enum Health {
    Ok,
    NotOk
}


#[derive(Deserialize, Clone, Serialize)]
pub struct Server {
    ip_address: IpAddr,
    port: u32,
    label: Option<String>,
    last_heartbeat: Option<SystemTime>,
    status: Option<Health>
}


impl Server {
    pub fn key(&self) -> String {
        [self.ip_address.to_string() + &self.port.to_string()].join("-")
    }

    pub fn add_heartbeat(&mut self) -> () {
        self.last_heartbeat = Some(SystemTime::now());
        self.status = Some(Health::Ok);
    }
}