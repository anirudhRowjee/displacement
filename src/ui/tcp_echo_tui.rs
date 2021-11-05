extern crate crossterm;

use std::{error::Error, io};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, List, ListItem, ListState};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color, Modifier};
use tui::text::{Span, Spans};


use std::sync::mpsc;


pub fn sanitize_byte_array(bytes: [u8; 1024]) -> String {
    let rec = String::from_utf8(bytes.to_vec()).unwrap();
    return rec.trim_matches(char::from(0)).trim().to_string();
}

pub async fn render_ui(reciever: mpsc::Receiver<[u8; 1024]>) -> Result<(), io::Error> {
    println!("hello, world!");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().unwrap();

    // generic list of strings for the echo data to be dumped
    let mut items = Vec::new();
    let mut list_state = ListState::default();
    let mut quit = false;

    loop {

        let recieved = reciever.recv().unwrap();
        let sanitized = sanitize_byte_array(recieved);

        if "closed".eq(&sanitized) {
            quit = true;
        }

        items.push(sanitized);


        if quit {
            terminal.clear().unwrap();
            break;
        } else {
            let new_items: Vec<ListItem> = items
                .iter()
                .map(|item_text| {
                    ListItem::new(Spans::from(item_text.as_str()))
                        .style(Style::default().fg(Color::White).bg(Color::Black))
                })
                .collect();

            terminal.clear().unwrap();

            terminal.draw(|f| {

                // setup the responsive layout of the terminal window
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10) // this is just whitespace
                        ].as_ref()
                    )
                    .split(f.size());

                let block = Block::default()
                    .title("TUI Echo Server")
                    .borders(Borders::ALL);

                f.render_widget(block, chunks[0]);

                let block2 = Block::default()
                    .title("Block 2")
                    .borders(Borders::ALL);

                let messages = List::new(new_items)
                    .block(block2)
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                    .highlight_symbol(">>");

                f.render_stateful_widget(messages, chunks[1], &mut list_state);

            })?;
        }

    };

    Ok(())
}
