use std::error::Error;
use std::path::PathBuf;

use mods::buffer::Buffer;
use mods::terminal::Terminal;

pub mod mods;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut term = Terminal::build()?;

    let filepath = PathBuf::from("tests/from_file_test.txt");
    let buffer = Buffer::from_file(filepath)?;
    let filename = buffer.get_file_name();
    let mut secu = 0;
    loop {
        term.clear()?;
        term.write_status_line(&filename)?;
        term.write_buffer(&buffer)?;
        secu += 1;
        if secu > 1000 {
            break;
        }
    }

    Ok(())
}
