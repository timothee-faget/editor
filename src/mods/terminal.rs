use crossterm::{cursor, terminal, ExecutableCommand};

use std::{error::Error, io};

use super::buffer::TextBuffer;

pub struct Terminal {
    stdout: io::Stdout,
    offset: u16,
    mode: TerminalMode,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            stdout: io::stdout(),
            offset: 0,
            mode: TerminalMode::Classic,
        }
    }

    pub fn build() -> Result<Terminal, Box<dyn Error>> {
        let mut term = Terminal {
            stdout: io::stdout(),
            offset: 0,
            mode: TerminalMode::Classic,
        };
        term.switch_mode()?;
        term.enter_alternate()?;
        term.clear()?;
        Ok(term)
    }

    pub fn switch_mode(&mut self) -> Result<&Self, Box<dyn Error>> {
        match self.mode {
            TerminalMode::Raw => {
                terminal::disable_raw_mode()?;
                self.stdout.execute(cursor::Show)?;
                self.mode = TerminalMode::Classic;
                Ok(self)
            }
            TerminalMode::Classic => {
                terminal::enable_raw_mode()?;
                self.stdout.execute(cursor::Hide)?;
                self.mode = TerminalMode::Raw;
                Ok(self)
            }
        }
    }

    pub fn enter_alternate(&mut self) -> Result<(), Box<dyn Error>> {
        self.stdout.execute(terminal::EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate(&mut self) -> Result<(), Box<dyn Error>> {
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        self.offset = 0;
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;
        self.stdout.execute(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn move_to_line(&mut self, line: u16) -> Result<(), Box<dyn Error>> {
        self.stdout.execute(cursor::MoveTo(0, line))?;
        Ok(())
    }

    pub fn print_app(&mut self) -> Result<(), Box<dyn Error>> {
        println!("editor");
        self.offset += 1;
        Ok(())
    }

    pub fn print_buffer(&mut self, buffer: &TextBuffer) -> Result<(), Box<dyn Error>> {
        for line in buffer.to_string().lines() {
            self.move_to_line(self.offset)?;
            println!("{}", line);
            self.offset += 1;
        }
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        if self.mode == TerminalMode::Raw {
            self.switch_mode().unwrap();
        }
        self.leave_alternate().unwrap();
    }
}

#[derive(Debug, PartialEq)]
enum TerminalMode {
    Raw,
    Classic,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_terminal() {
        let term = Terminal::new();

        assert!(term.offset == 0)
    }

    #[test]
    fn build_terminal() {
        let term = Terminal::build();

        assert!(term.is_ok())
    }

    #[test]
    fn switch_mode() {
        let mut term = Terminal::new();

        assert!(term.switch_mode().is_ok());
        assert_eq!(term.mode, TerminalMode::Raw);
        assert!(term.switch_mode().is_ok());
        assert_eq!(term.mode, TerminalMode::Classic)
    }

    #[test]
    fn alterate_screen() {
        let mut term = Terminal::new();

        assert!(term.enter_alternate().is_ok());
        assert!(term.leave_alternate().is_ok())
    }

    #[test]
    #[ignore]
    fn clear_terminal() {
        let mut term = Terminal::new();

        assert!(term.clear().is_ok());
        assert_eq!(term.offset, 0)
    }

    #[test]
    fn move_to_line() {
        let mut term = Terminal::build().unwrap();
        assert!(term.move_to_line(0).is_ok());
        assert!(term.move_to_line(2).is_ok());
    }
}
