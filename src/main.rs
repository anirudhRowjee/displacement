/*
 * Displacement: A simple implementation of the FTP protocol
 */
use std::sync::mpsc;
use std::thread;
use std::net::{Ipv4Addr, IpAddr, SocketAddr};

mod ui;
mod network;


#[tokio::main]
async fn main() {

    // TODO add crate `clap` to parse IP from command line input
    // declare the FTP Server IP Address
    let server_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let (transmit, recieve) = mpsc::channel();

    let handle = thread::spawn(move || {
        if let Err(e) = network::run_listener(transmit, server_address) {
            println!("Error! Here it is > {:?}", e);
        }
    });

    println!("hello, world!");

    // ui::render_ui(recieve).await;

    handle.join().unwrap();
}

