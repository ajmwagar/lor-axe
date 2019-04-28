use loraxe::*;
use std::time::Duration;
use std::net::{SocketAddr, ToSocketAddrs};

fn config() -> Config {
    Config {
        https: false,
        sock_timeout: Duration::from_secs(10),
        delay: 15,
        addr: "google.com".to_string(),
        port: 80,
        rand_ua: true,
        read_size: 32,
        socket_count: 5,
        dos_type: DOSType::SlowLoris,
   }
}

#[test]
/// Create a single socket
fn create_socket() {

    let config = config();

    let url = [config.addr.clone(), config.port.to_string()].join(":");

    let sock_addr: SocketAddr = url.to_socket_addrs().unwrap().collect::<Vec<SocketAddr>>()[0];
    let result = init_socket(&config, &sock_addr);

    println!("{:?}", result);

    assert!(result.is_ok())
}

#[test]
/// Create a new lori
fn new_loraxe() {
    let config = config();
    let mut loraxe = Loraxe::new(config);

    assert!(true);
}

#[test]
/// Create new lori sockets
fn loraxe_socks() {
    let config = config();
    let mut loraxe = Loraxe::new(config);

    assert!(loraxe.create_sockets().is_ok());
}

#[test]
/// Create new lori sockets
fn loraxe_attack() {
    let config = config();
    let mut loraxe = Loraxe::new(config);

    assert!(loraxe.create_sockets().is_ok());
}
