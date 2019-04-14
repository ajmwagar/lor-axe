use lori::*;

const config = Config {
    https: false,
    delay: 15,
    addr: 0.0.0.0,
    port: 8000,
    rand_ua: true,
    socket_count: 150,
    dos_type: DOSType::SlowLoris,
}

#[test]
/// Create a single socket
fn create_socket() {
    assert!(init_socket(&config).is_ok())
}
