use lori::*;

fn config() -> Config {
    Config {
        https: false,
        delay: 15,
        addr: "0.0.0.0".to_string(),
        port: 8000,
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
    assert!(lori::init_socket(&config).is_ok())
        // unimplemented!()
}

#[test]
/// Create a new lori
fn new_lori() {
    let config = config();
    let mut lori = Lori::new(config);

    // assert_eq!(lori, lori);
    assert!(true);
}

#[test]
/// Create new lori sockets
fn lori_socks() {
    let config = config();
    let mut lori = Lori::new(config);

    assert!(lori.create_sockets().is_ok());
}

#[test]
/// Create new lori sockets
fn lori_attack() {
    let config = config();
    let mut lori = Lori::new(config);

    assert!(lori.create_sockets().is_ok());
    // assert!(lori.attack().is_ok());
}
