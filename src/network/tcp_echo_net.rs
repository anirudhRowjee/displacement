use std::net::IpAddr;
use std::sync::mpsc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use colored::*;

use std::str;


enum FtpRes {
    Syntax(String),      // responses with X0Z
    Information(String), // responses with X1Z
    Connection(String),  // responses with X2Z
    Auth(String),        // responses with X3Z
    Filesystem(String),  // responses with X5Z
    Error(String),
}

/// Function to decode a raw FTP Client response and identify it's type
fn decode_response(reply: &Vec<u8>) -> FtpRes {
    // dbg!(reply);
    let decoded_response: String = String::from(str::from_utf8(&reply).expect("Decode Failed!"));

    if reply.len() != 0 {
        match reply[1] {
            50 => FtpRes::Syntax(decoded_response),
            51 | 49 => FtpRes::Information(decoded_response),
            52 => FtpRes::Connection(decoded_response),
            53 => FtpRes::Auth(decoded_response),
            55 => FtpRes::Filesystem(decoded_response),
            _ => FtpRes::Error(decoded_response),
        }
    } else {
        return FtpRes::Error("No response from Server :(".to_string());
    }
}

/// function to dispatch the appropriate CLI response
/// based on the response code
fn decoded_response_demux(response: FtpRes) {

    match response {
        FtpRes::Syntax(decoded_response) => println!("SYNTAX: {}", decoded_response),
        FtpRes::Information(decoded_response) => {
            println!("INFORMATION: {}", decoded_response.green())
        }
        FtpRes::Connection(decoded_response) => {
            println!("CONNECTION: {}", decoded_response.yellow())
        }
        FtpRes::Auth(decoded_response) => println!("AUTH: {:?}", decoded_response.cyan()),
        FtpRes::Filesystem(decoded_response) => {
            println!("FILESYSTEM: {}", decoded_response.blue())
        }
        FtpRes::Error(decoded_response) => {
            println!("ERROR: {}", decoded_response.red());
        }
    }

}

// declare the main runtime as asynchronous
#[tokio::main]
pub async fn run_listener(
    _transmitter: mpsc::Sender<[u8; 1024]>,
    _server_address: IpAddr,
) -> Result<(), Box<dyn std::error::Error>> {

    /*
     * a TCPListener is like a TCP "server" - it allows us to read and write packets
     * at this specific address, which acts as a TCP Interface. Here, we're binding this listener
     * to port 8080.
     */

    // replacing the listener with a stream to ensure bidirectional communication
    // let listener = TcpListener::bind("127.0.0.1:8080").await?;

    //TODO setup better error handling for the connection failure
    let mut stream = TcpStream::connect("127.0.0.1:21").await?;

    println!("Listening to FTP Server located at 127.0.0.1:21");

    // spawn the data PI

    let client_future = tokio::spawn(async move {
        // println!("Hello, from the Client PI!");

        // spawn the client PI
        let mut databuf = Vec::with_capacity(1024);
        let mut command_string: String = String::from("");

        loop {

            // we read from the stream here
            // println!("About to start reading");

            // println!("About to start reading! Waiting for the socket");

            stream.readable().await.unwrap();
            // let bytes_read = stream.try_read(&mut databuf).unwrap();
            stream.read_buf(&mut databuf).await.unwrap();

            // dbg!(bytes_read);

            // println!("In the loop!");

            // println!("About to start reading to buffer! Waiting for the socket");

            // we check for errors here
            // find out what the response says, assume ASCII mode
            let decoded_response = decode_response(&databuf);

            decoded_response_demux(decoded_response);

            // TODO setup handlers
            // TODO setup an enum response type for nothing coming back from the server
            println!("D >> ");

            std::io::stdin().read_line(&mut command_string).unwrap();
            println!("Executing >> {:?}", command_string);

            stream.writable().await.unwrap();
            stream.try_write(command_string.as_bytes()).unwrap();

            command_string.clear();
            databuf.clear();
        }
    });

    // spawn the data PI
    let server_future = tokio::spawn(async move {

        // println!("Hello, from the data transfer protocol");
        let mut databuf: Vec<u8> = Vec::with_capacity(4096);

        // why 2561? It's easy to set it up using the PORT Command
        // making us use PORT 127,0,0,1,10,1 to tell the FTP server to connect to our PI
        let datastream = TcpListener::bind("127.0.0.1:2561").await.unwrap();

        // println!("Data Listening on 127.0.0.1:1027");

        loop {
            match datastream.accept().await {
                Ok((mut datasocket, addr)) => {
                    // println!("New Client! {:?}", addr);
                    datasocket.readable().await.unwrap();
                    datasocket.read_to_end(&mut databuf).await.unwrap();
                    // println!("Server Says >> {:?}", databuf);

                    let decoded_response = decode_response(&databuf);

                    // TODO setup handlers
                    // TODO setup an enum response type for nothing coming back from the server
                    decoded_response_demux(decoded_response);

                    databuf.clear();
                }
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    });

    client_future.await.unwrap();
    server_future.await.unwrap();

    Ok(())
}
