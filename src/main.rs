/*
 * Displacement: A simple implementation of the FTP protocol
 */

/*
use std::process;
use termion::input::TermRead;
use tokio::net::TcpListener;
use std::io;
use termion::raw::IntoRawMode;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use termion::event::Key;
use std::io::{Write, stdout, stdin};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

/*
 *
 *#[tokio::main]
 *async fn ping() -> (Result<() , Box<dyn std::error::Error>>) {
 *    let literal = TcpListener::bind("127.0.0.1:3000")?;
 *}
 * Doesnt work cus asyn always returns a box type () , async is rust works more like a closure
 */
#[tokio::main]
async fn ping() -> Result<() , Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:3000").await?;
        let (mut socket , addr) = listener.accept().await?;
        let rand : u8 = 0;
        let mut emptbuff : Vec<u8> = vec![0;1024];
        let res = socket.read_to_end(&mut emptbuff).await?;

        println!("I've been pinged!");
    Ok(())
}


fn main() -> Result<(), io::Error> {
    println!("Hello, world!");

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let text = vec![
        Spans::from(Span::styled(
            "FTP using Rust",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
    ];
    loop{
    terminal.draw(|f| {
        // split the layout into three vertical chunks according
        // to the constraints provided
        let para = Paragraph::new(text.clone())
            .block(Block::default().title("Hello world block").borders(Borders::ALL))  
            .style(Style::default().bg(Color::White))
            .alignment(Alignment::Center);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(f.size());

        // create a basic block
        let block = Block::default().title("Block").borders(Borders::ALL);

        // render
        f.render_widget(para, chunks[0]);

        let block = Block::default()
            .title("Block 2")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::White));

        // render
        f.render_widget(block, chunks[1]);

        let block = Block::default()
            .title("Block 3")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::White));

        // render
        f.render_widget(block, chunks[2]);

        //expecting inouts after render
         })?;
        let stdin = stdin();
        for c in stdin.keys() {
             match c.unwrap() {
                 Key::Char('q') => {
                     println!("reached here");
                    process::exit(1);
                 },
                 Key::Char('s')=>{
                    let res = ping();
                    println!("{:?}",res);
                 }
            _ => {}

    }
        };
}}
*/

use std::thread;
use std::time::Duration;
// MPSC => Multi Producer Single Consumer
use std::sync::mpsc;

fn main() {

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    let child = thread::spawn(move || {

        let vals = vec![
            String::from("Hi!"),
            String::from("From!"),
            String::from("The!"),
            String::from("Thread!"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });


    let child2 = thread::spawn(move || {

        let vals = vec![
            String::from("2: Hi!"),
            String::from("2: From!"),
            String::from("2: The!"),
            String::from("2: Thread!"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    for recieved in rx {
        println!("Got {}", recieved);
    }

    child.join().unwrap();
    child2.join().unwrap();

}

