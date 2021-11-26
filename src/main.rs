/*
 * Displacement: A simple implementation of the FTP protocol
 */
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::mpsc;
use std::thread;

mod network;
mod ui;

struct State {
    username: String,
    data_port: u32,
    control_port: u32,
    server_ip: String,
    recieve_time: String,
    recieve_date: String,
    file_size: f64,
    source_directory: String,
    current_driectory: String,
}

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
