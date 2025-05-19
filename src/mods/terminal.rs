use crossterm::{cursor, terminal, ExecutableCommand};

use std::{error::Error, io};

pub struct Terminal {
    stdout: io::Stdout,
    offset: u16,
    mode: TerminalMode,
}

impl Drop for Terminal {
    fn drop(&mut self) {
        if self.mode == TerminalMode::Raw {
            self.switch_mode().unwrap();
        }
        self.leave_alternate().unwrap();
    }
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

    pub fn print(&mut self, string: String) -> Result<(), Box<dyn Error>> {
        self.move_to_line(0)?;
        println!("{string}");
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum TerminalMode {
    Raw,
    Classic,
}
