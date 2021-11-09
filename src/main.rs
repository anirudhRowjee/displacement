/*
 * Displacement: A simple implementation of the FTP protocol
 */
use std::sync::mpsc;
use std::thread;
mod ui;
mod network;

struct State {
    username: String.
    data_port: u32,
    control_port: u32,
    server_ip: String,
    recieve_time: String,
    recieve_date: String,
    file_size: f64,
    source_directory: String,
    current_driectory: String
}

#[tokio::main]
async fn main() {

    let (transmit, recieve) = mpsc::channel();

    let handle = thread::spawn(|| {
        if let Err(e) = network::run_listener(transmit) {
            println!("Error! Here it is > {:?}", e);
        }
    });

    println!("hello, world!");

    ui::render_ui(recieve).await;

    handle.join().unwrap();
}

