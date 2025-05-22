use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand,
};

use std::{error::Error, io};

use super::{buffer::Buffer, cursor::Cursor};

pub struct Terminal {
    stdout: io::Stdout,
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
            mode: TerminalMode::Classic,
        }
    }

    pub fn build() -> Result<Terminal, Box<dyn Error>> {
        let mut term = Terminal {
            stdout: io::stdout(),
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

    pub fn write_status_line(
        &mut self,
        filename: &String,
        cursor: &Cursor,
    ) -> Result<(), Box<dyn Error>> {
        let size = self.get_size()?;

        // Backgound
        let style = CharStyle::new(Color::White, Color::Grey);
        for w in 0..size.0 {
            self.write(' ', &style, (w as u16, size.1))?;
        }

        // Mode
        let style_b1 = CharStyle::new(Color::White, Color::DarkGreen);
        self.write_block(&String::from(" NORMAL "), &style_b1, (0, size.1))?;

        // Filename
        let style_b2 = CharStyle::new(Color::White, Color::DarkBlue);
        self.write_block(&filename, &style_b2, (8, size.1))?;

        // position
        let style_b3 = CharStyle::new(Color::White, Color::DarkRed);
        let position = cursor.get_pos();
        let position_string = String::from(format!(
            "{:?} {}:{} {} ",
            cursor.get_prev_pos(),
            position.1,
            position.0,
            cursor.get_opt_col()
        ));
        self.write_block(
            &position_string,
            &style_b3,
            (size.0 - position_string.len() as u16, size.1),
        )?;

        Ok(())
    }

    pub fn update_status_line_cursor(&mut self, cursor: &Cursor) -> Result<(), Box<dyn Error>> {
        let size = self.get_size()?;
        let style_b3 = CharStyle::new(Color::White, Color::DarkRed);
        let position = cursor.get_pos();
        let position_string = String::from(format!(
            "{:?} {}:{} {} ",
            cursor.get_prev_pos(),
            position.1,
            position.0,
            cursor.get_opt_col()
        ));
        self.write_block(
            &position_string,
            &style_b3,
            (size.0 - position_string.len() as u16, size.1),
        )?;

        Ok(())
    }

    pub fn write_lines(
        &mut self,
        lines: &Vec<(u16, String)>,
        number_col_width: u16,
        buffer_size: u16,
    ) -> Result<(), Box<dyn Error>> {
        let number_col_style = CharStyle::new(Color::Grey, Color::DarkGrey);
        let line_style = CharStyle::new(Color::White, Color::Black);
        let size = self.get_size()?;
        for (i, line) in lines.iter().enumerate() {
            if line.0 < buffer_size {
                self.write_block(
                    &format!("{:>width$}", &line.0 + 1, width = number_col_width as usize),
                    &number_col_style,
                    (0, i as u16),
                )?;
                self.write_block(
                    &format!(
                        " {} {:>width$}",
                        &line.1,
                        ' ',
                        width = size.0 as usize - line.1.len()
                    ),
                    &line_style,
                    (number_col_width as u16, i as u16),
                )?;
            } else {
                self.write_block(
                    &format!("{:>width$}", "", width = number_col_width as usize),
                    &number_col_style,
                    (0, i as u16),
                )?;
                self.write_block(
                    &format!("{:>width$}", " ", width = size.0 as usize),
                    &line_style,
                    (number_col_width, i as u16),
                )?;
            }
        }

        Ok(())
    }

    pub fn write_buffer(&mut self, buffer: &Buffer) -> Result<(), Box<dyn Error>> {
        let size = self.get_size()?;

        let lines = buffer.get_n_lines(size.1 as usize - 1, 0);

        let buffer_size = buffer.get_size();

        let number_col_width = buffer.get_size().to_string().len() + 1;

        let number_col_style = CharStyle::new(Color::Grey, Color::DarkGrey);
        let line_style = CharStyle::new(Color::White, Color::Black);
        for (i, line) in lines.iter().enumerate() {
            if i < buffer_size {
                self.write_block(
                    &format!("{:>width$}", &line.0, width = number_col_width),
                    &number_col_style,
                    (0, i as u16),
                )?;
                self.write(' ', &line_style, (number_col_width as u16, i as u16))?;
                self.write_block(
                    &line.1,
                    &line_style,
                    (number_col_width as u16 + 1, i as u16),
                )?;
            } else {
                self.write_block(
                    &format!("{:>width$}", "", width = number_col_width),
                    &number_col_style,
                    (0, i as u16),
                )?;
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

        prev_pos.0 += number_col_width + 1;
        self.write(
            *prev_char,
            &CharStyle::new(Color::White, Color::Black),
            prev_pos,
        )?;

        let char = &lines[pos.1 as usize]
            .1
            .chars()
            .nth(pos.0 as usize)
            .map_or(' ', |v| v);
        pos.0 += number_col_width + 1;
        self.write(*char, &CharStyle::new(Color::Black, Color::White), pos)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum TerminalMode {
    Raw,
    Classic,
}

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
