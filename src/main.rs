/*
 * Displacement: A simple implementation of the FTP protocol
 */

use std::io;
use termion::raw::IntoRawMode;

use tui::backend::TermionBackend;
use tui::Terminal;
// use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};

fn main() -> Result<(), io::Error> {
    println!("Hello, world!");

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        // split the layout into three vertical chunks according
        // to the constraints provided
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80)
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());

        // create a basic block
        let block = Block::default().title("Block").borders(Borders::ALL);

        // render
        f.render_widget(block, chunks[0]);

        let block = Block::default().title("Block 2").borders(Borders::ALL);

        // render
        f.render_widget(block, chunks[1]);

        let block = Block::default().title("Block 3").borders(Borders::ALL);

        // render
        f.render_widget(block, chunks[2]);
    })?;
    Ok(())
}
