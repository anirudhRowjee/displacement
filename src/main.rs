/*
 * Displacement: A simple implementation of the FTP protocol
 */
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::mpsc;
use std::thread;
use colored::*;

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

fn print_logo_banner()
{
    let logo = r#"
   ___  ___________  __   ___  ___________  ________  ________
  / _ \/  _/ __/ _ \/ /  / _ |/ ___/ __/  |/  / __/ |/ /_  __/
 / // // /_\ \/ ___/ /__/ __ / /__/ _// /|_/ / _//    / / /   
/____/___/___/_/  /____/_/ |_\___/___/_/  /_/___/_/|_/ /_/    
                                                              
    "#;
    println!("{}", logo.blue());
    println!("{}", "Displacement is a CLI FTP Client".red());
}


#[tokio::main]
async fn main() {

    // TODO add crate `clap` to parse IP from command line input
    // declare the FTP Server IP Address

    print_logo_banner();

    let server_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let (transmit, recieve) = mpsc::channel();

    let handle = thread::spawn(move || {
        if let Err(e) = network::run_listener(transmit, server_address) {
            println!("Error! Here it is > {:?}", e);
        }
    });

    // println!("hello, world!");
    // ui::render_ui(recieve).await;

    handle.join().unwrap();
}
