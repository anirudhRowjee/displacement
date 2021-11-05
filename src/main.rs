/*
 * Displacement: A simple implementation of the FTP protocol
 */
use std::sync::mpsc;
use std::thread;
mod ui;
mod network;


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

