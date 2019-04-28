extern crate pretty_env_logger;
#[macro_use] extern crate log;
use std::env;
use rayon::prelude::*;
use std::net::UdpSocket;
use rand::{ThreadRng, Rng};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>>{
    //Set the `RUST_LOG` var if none is provided
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "loraxe=INFO");
    }

    pretty_env_logger::init_timed();

    static IP: &str = &"127.0.0.1";

    let mut rng = rand::thread_rng();

    let bytes = (0..1024).into_iter().map(|i| {
        rng.gen::<u8>()
    }).collect::<Vec<u8>>();


    let sock = UdpSocket::bind("0.0.0.0:1337").expect("Couldn't bind to address");

    loop {
        println!("Sending packet");

        (0..65535).into_par_iter().for_each(|i: i32|{
            let mut url = String::with_capacity(IP.len() + i.to_string().len());
            url.push_str(IP);
            url.push_str(":");
            url.push_str(&i.to_string());

            // println!("Sending Packet to port {}", i);
            sock.send_to(&bytes, IP);
        })
    }
}
