use serde::{Deserialize, Serialize};
use std::net::{IpAddr};
use std::time::{Duration, SystemTime};
use uuid::{uuid, Uuid};



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
    timeout: u32,
    period: Duration,
    endpoint: String,
    max_attempts: u32,
    health_status: HealthStatus,
}


#[derive(Deserialize, Clone, Serialize)]
pub struct Server {
    port: u32,
    id: Uuid,
    ip_address: IpAddr,
    label: Option<String>,
    health_check_configuration: Option<HealthCheckConfiguration>
}


impl Server {
    pub fn key(&self) -> String {
        [self.ip_address.to_string() + &self.port.to_string()].join("-")
    }
}