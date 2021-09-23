/*
 * Displacement: A simple implementation of the FTP protocol
 */

use std::io;
use termion::raw::IntoRawMode;

use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    println!("Hello, world!");

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let text = vec![
        Spans::from(""),
        Spans::from(""), //simply to push content into bloc, very bad practice.
        Spans::from(Span::styled(
            "FTP using Rust",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
    ];
    terminal.draw(|f| {
        // split the layout into three vertical chunks according
        // to the constraints provided
        let para = Paragraph::new(text.clone())
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
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::White));

        // render
        f.render_widget(block, chunks[0]);

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
        f.render_widget(para, chunks[0]);
    })?;
    Ok(())
}
