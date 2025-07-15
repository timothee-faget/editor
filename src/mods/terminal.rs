use crossterm::{
    cursor,
    style::{style, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self},
    ExecutableCommand,
};

use std::{error::Error, io, rc::Rc};

use super::{config::Config, cursor::Cursor, numcol::NumColumn, statusline::StatusLine};

pub struct Terminal {
    stdout: io::Stdout,
    mode: TerminalMode,
    config: Rc<Config>,
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
            mode: TerminalMode::Classic,
            config: Rc::new(Config::new()),
        }
    }

    pub fn build(config: Rc<Config>) -> Result<Terminal, Box<dyn Error>> {
        let mut term = Terminal {
            stdout: io::stdout(),
            mode: TerminalMode::Classic,
            config,
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
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;
        self.stdout.execute(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn get_size(&self) -> Result<(u16, u16), Box<dyn Error>> {
        let size = terminal::size()?;
        Ok(size)
    }

    pub fn move_to_line(&mut self, line: u16) -> Result<(), Box<dyn Error>> {
        self.stdout.execute(cursor::MoveTo(0, line))?;
        Ok(())
    }

    pub fn move_to(&mut self, position: (u16, u16)) -> Result<(), Box<dyn Error>> {
        self.stdout
            .execute(cursor::MoveTo(position.0, position.1))?;
        Ok(())
    }

    pub fn print(&mut self, string: String) -> Result<(), Box<dyn Error>> {
        self.move_to_line(0)?;
        println!("{string}");
        Ok(())
    }

    pub fn write(
        &mut self,
        ch: char,
        style: &CharStyle,
        position: (u16, u16),
    ) -> Result<(), Box<dyn Error>> {
        
        self.move_to(position)?;
        self.stdout.execute(SetBackgroundColor(style.bg()))?;
        self.stdout.execute(SetForegroundColor(style.fg()))?;
        // self.stdout.execute(SetBackgroundColor(self.config.styles.buffer.fg))?;
        self.stdout.execute(Print(ch))?;
        self.stdout.execute(ResetColor)?;
        Ok(())
    }

    pub fn write_block(
        &mut self,
        text: &String,
        style: &CharStyle,
        position: (u16, u16),
    ) -> Result<(), Box<dyn Error>> {
        for (i, ch) in text.chars().into_iter().enumerate() {
            if let Err(e) = self.write(ch, style, (position.0 + i as u16, position.1)) {
                if i == 0 {
                    return Err(e);
                } else {
                    eprintln!("Block out of bounds");
                    return Ok(());
                }
            }
        }
        Ok(())
    }


    pub fn draw_cursor(
        &mut self,
        lines: &Vec<(u16, String)>,
        cursor: &Cursor,
        number_col_width: u16,
    ) -> Result<(), Box<dyn Error>> {
        let mut pos = cursor.get_pos();
        let mut prev_pos = cursor.get_prev_pos();

        let prev_char = &lines[prev_pos.1 as usize]
            .1
            .chars()
            .nth(prev_pos.0 as usize)
            .map_or(' ', |v| v);

        prev_pos.0 += number_col_width;
        let style = self.config.styles.cursor;
        self.write(*prev_char, &style, prev_pos)?;

        let char = &lines[pos.1 as usize]
            .1
            .chars()
            .nth(pos.0 as usize)
            .map_or(' ', |v| v);
        pos.0 += number_col_width;
        
        let style = self.config.styles.buffer;
        self.write(*char, &style, pos)?;

        Ok(())
    }

    pub fn draw_numcol(&mut self, numcol: &NumColumn) -> Result<(), Box<dyn Error>> {
        let width = numcol.get_width() - 2;
        let style = self.config.styles.num_col;
        for (i, line) in numcol.get_nums().iter().enumerate() {
            if let Some(l) = line {
                self.write_block(
                    &format!(" {:>width$} ", l, width = width as usize),
                    &style,
                    (0, i as u16),
                )?;
            } else {
                self.write_block(
                    &format!(" {:>width$} ", '.', width = width as usize),
                    &style,
                    (0, i as u16),
                )?;
            }
        }
        Ok(())
    }

    pub fn draw_status_line(&mut self, status_line: &StatusLine) -> Result<(), Box<dyn Error>> {
        let size = self.get_size()?;
        let style = self.config.styles.status_line;
        for w in 0..size.0 {
            self.write(' ', &style, (w as u16, size.1))?;
        }

        let blocks = status_line.get_blocks();
        self.write_block(
            &blocks[0],
            &CharStyle::new(Color::Grey, Color::Green),
            (0, size.1),
        )?;

        let mut pos = blocks[0].len() as u16;
        self.write_block(
            &blocks[1],
            &CharStyle::new(Color::Grey, Color::Blue),
            (pos, size.1),
        )?;

        pos = size.0 - blocks[2].len() as u16;
        self.write_block(
            &blocks[2],
            &CharStyle::new(Color::Grey, Color::DarkCyan),
            (pos, size.1),
        )?;

        Ok(())
    }

    // pub fn update_status_line(&mut self, status_line: &StatusLine) -> Result<(), Box<dyn Error>> {
    //     Ok(())
    // }

    pub fn draw_buffer(
        &mut self,
        lines: &Vec<(u16, String)>,
        numcol: &NumColumn,
    ) -> Result<(), Box<dyn Error>> {
        let line_style = CharStyle::new(Color::White, Color::Black);
        let width = numcol.get_width();
        let buffer_width = self.get_size()?.0 - width;
        for (i, line) in lines.iter().enumerate() {
            let line_len = line.1.len();
            self.write_block(&format!("{}", line.1), &line_style, (width, i as u16))?;
            self.write_block(
                &format!(
                    " {:>width$} ",
                    '.',
                    width = buffer_width as usize - line_len as usize
                ),
                &line_style,
                (width + line_len as u16, i as u16),
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum TerminalMode {
    Raw,
    Classic,
}

#[derive(Clone, Copy)]
pub struct CharStyle {
    fg: Color,
    bg: Color,
}

impl CharStyle {
    pub fn new(fg: Color, bg: Color) -> Self {
        Self { fg, bg }
    }

    pub fn fg(&self) -> Color {
        self.fg
    }

    pub fn bg(&self) -> Color {
        self.bg
    }
}
