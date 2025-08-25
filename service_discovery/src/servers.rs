mod servers;

use serde::Deserialize;

#[derive(Deserialize)]
struct Server {
    ip_address: IpAddr,
    port: u32,
    label: String
}

#[derive(Deserialize)]
struct Host {
    ip_address: IpAddr
}

#[derive(Deserialize)]
struct Status {
    healthy:  bool
}


fn get_availble_services(label: String) -> Vector[Server] {
    // Se comunica com a base redis para encontrar os serviços disponiveis e abre um pool tcp para cada um deles.
}