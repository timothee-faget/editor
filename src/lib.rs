use std::error::Error;

use crossterm::event::{self, Event, KeyCode};
use mods::{buffer::TextBuffer, terminal::Terminal};

pub mod mods;

pub fn run_editor() -> Result<(), Box<dyn Error>> {
    let mut terminal = Terminal::build()?;

    let mut buffer = TextBuffer::from_file("help.txt")?;

    // terminal.print_buffer(buffer)?;

    loop {
        terminal.clear()?;
        terminal.print_buffer(&buffer)?;
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    buffer.push('a');
                }
                _ => {}
            }
        }
    }

    Ok(())
}
