use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::mpsc;


// declare the main runtime as asynchronous
#[tokio::main]
pub async fn run_listener(transmitter: mpsc::Sender<[u8; 1024]>) -> Result<(), Box<dyn std::error::Error>> {
    /* 
     * a TCPListener is like a TCP "server" - it allows us to read and write packets
     * at this specific address, which acts as a TCP Interface. Here, we're binding this listener
     * to port 8080.
     */
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on 127.0.0.1:8080");

    loop {
        let tx = transmitter.clone();
        // accept our interfaces as soon as a connection takes place
        let (mut socket, _) = listener.accept().await?;

        /*
         * create a "task", which is the atomic unit of execution in a Tokio Application.
         * These tasks may run (depending on the scheduler and/or runtime) on the same, or on a
         * different thread.
         */
        tokio::spawn(async move {

            // state!
            let mut databuf = [0; 1024];

            loop {
                let n = match socket.read(&mut databuf).await {
                    // if n is zero, the socket is closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Couldn't read! Error: {:?}", e);
                        return;
                    }
                };

                // since we're an echo server, we write this data back

                // copy the data buffer and send it to the UI
                tx.send(databuf).unwrap();

                if let Err(e) = socket.write_all(&databuf[0..n]).await {
                    eprintln!("Couldn't Write! Error : {:?}", e);
                    return;
                }
            }

        });
    }
}


