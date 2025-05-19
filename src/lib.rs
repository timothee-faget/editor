use std::error::Error;
use std::{thread, time::Duration};

use mods::terminal::Terminal;

pub mod mods;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut term = Terminal::build()?;

    term.print(String::from("Salut tout le monde"))?;

    thread::sleep(Duration::from_secs(2));

    Ok(())
}
