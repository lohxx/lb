use serde::{Deserialize, Serialize};
use std::net::{IpAddr};
use std::time::SystemTime;

#[derive(Deserialize, Clone, Serialize)]
pub enum Health {
    Ok,
    NotOk
}

#[derive(Deserialize, Clone, Serialize)]
pub struct HealthStatus {
    status: Health,
    last_succesful_check: Option<SystemTime>,
    last_error_status: u64
}

#[derive(Deserialize, Clone, Serialize)]
pub struct HealthCheckConfiguration {
    period: u64,
    endpoint: String,
    health_status: HealthStatus
}


#[derive(Deserialize, Clone, Serialize)]
pub struct Server {
    port: u32,
    ip_address: IpAddr,
    label: Option<String>,
    health_check_configuration: Option<HealthCheckConfiguration>
}


impl Server {
    pub fn key(&self) -> String {
        [self.ip_address.to_string() + &self.port.to_string()].join("-")
    }
}