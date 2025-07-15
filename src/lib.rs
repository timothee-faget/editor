use std::cell::RefCell;
use std::error::Error;
use std::path::PathBuf;
use std::rc::Rc;

use crossterm::event::{self, Event, KeyCode};
use mods::buffer::Buffer;
use mods::config::Config;
use mods::cursor::Cursor;
use mods::numcol::NumColumn;
use mods::statusline::StatusLine;
use mods::terminal::Terminal;

pub mod mods;

pub fn run_editor() -> Result<(), Box<dyn Error>> {
    let mut editor = Editor::build(PathBuf::from("tests/test_editor_1.txt"))?;
    editor.run()?;

    Ok(())
}

pub struct Editor {
    terminal: Terminal,
    config: Rc<Config>,
    buffer: Rc<Buffer>,
    scroll_offset: usize,
    cursor: Rc<RefCell<Cursor>>,
    mode: EditorMode,
    lines: Vec<(u16, String)>,
    num_col: NumColumn,
    status_line: StatusLine,
}

impl Editor {
    pub fn build(filepath: PathBuf) -> Result<Self, Box<dyn Error>> {
        // TODO :  Rendre ça pluys intelligible et plus propre

        let config = Rc::new(Config::default());
        let terminal = Terminal::build(Rc::clone(&config))?;

        let buffer = Rc::new(Buffer::from_file(filepath)?);
        let buffer_size = buffer.get_size() as u16;
        let term_size = terminal.get_size().unwrap().1 - 1;
        let lines = buffer.get_n_lines(term_size as usize, 0);
        let cursor = Rc::new(RefCell::new(Cursor::new()));
        let mode = Rc::new(EditorMode::Normal);
        let scroll_offset = Rc::new(0);
        let status_line = StatusLine::new(
            mode,
            Rc::clone(&cursor),
            Rc::clone(&buffer),
            Rc::clone(&scroll_offset),
        );

        Ok(Self {
            terminal,
            config,
            buffer,
            scroll_offset: 0,
            cursor,
            mode: EditorMode::Normal,
            lines,
            num_col: NumColumn::build(buffer_size, 0, term_size),
            status_line,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.clear()?;
        self.display_numcol()?;
        self.display_status_line()?;
        self.display_buffer_lines()?;
        self.display_cursor()?;

        loop {
            if let Event::Key(key_event) = event::read()? {
                match self.mode {
                    EditorMode::Normal => match key_event.code {
                        KeyCode::Right | KeyCode::Char('l') => {
                            self.move_cursor_right()?;
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            self.move_cursor_left()?;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.move_cursor_up()?;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.move_cursor_down()?;
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    },
                    EditorMode::Insert => {}
                }
            }
        }
        Ok(())
    }

    fn move_cursor_right(&mut self) -> Result<(), Box<dyn Error>> {
        let current_pos = self.cursor.borrow().get_pos();
        let current_line_len = self.lines[current_pos.1 as usize].1.len();
        if current_line_len > 0 && current_pos.0 < current_line_len as u16 - 1 {
            self.cursor.borrow_mut().move_right();
            self.terminal.draw_cursor(
                &self.lines,
                &self.cursor.borrow(),
                self.num_col.get_width(),
            )?;

            self.display_status_line()?;
        }
        Ok(())
    }

    fn move_cursor_left(&mut self) -> Result<(), Box<dyn Error>> {
        self.cursor.borrow_mut().move_left();
        self.terminal
            .draw_cursor(&self.lines, &self.cursor.borrow(), self.num_col.get_width())?;
        self.display_status_line()?;
        Ok(())
    }

    fn move_cursor_up(&mut self) -> Result<(), Box<dyn Error>> {
        if self.cursor.borrow().get_pos().1 + self.scroll_offset as u16 > 0 {
            if self.cursor.borrow().get_pos().1 == 0 && self.scroll_offset > 0 {
                self.scroll_offset -= 1;
                self.lines = self
                    .buffer
                    .get_n_lines(self.lines.len(), self.scroll_offset);
                self.display_buffer_lines()?;
                self.terminal.draw_cursor(
                    &self.lines,
                    &self.cursor.borrow(),
                    self.num_col.get_width(),
                )?;
                self.display_status_line()?;
            } else {
                let next_h = self.cursor.borrow().get_pos().1 - 1;
                let mut max_col = self.lines[next_h as usize].1.len() as u16;
                if max_col > 0 {
                    max_col -= 1;
                }
                let opt_col = self.cursor.borrow().get_opt_col();
                if opt_col <= max_col {
                    self.cursor.borrow_mut().set_pos((opt_col, next_h));
                } else {
                    self.cursor.borrow_mut().set_pos((max_col, next_h));
                }
                self.terminal.draw_cursor(
                    &self.lines,
                    &self.cursor.borrow(),
                    self.num_col.get_width(),
                )?;
                self.display_status_line()?;
            }
        }
        Ok(())
    }

    fn move_cursor_down(&mut self) -> Result<(), Box<dyn Error>> {
        if self.cursor.borrow().get_pos().1
            < (self.buffer.get_size() - self.scroll_offset) as u16 - 1
        {
            if self.cursor.borrow().get_pos().1 == self.lines.len() as u16 - 1 {
                self.scroll_offset += 1;
                self.lines = self
                    .buffer
                    .get_n_lines(self.lines.len(), self.scroll_offset);
                self.display_buffer_lines()?;
                self.terminal.draw_cursor(
                    &self.lines,
                    &self.cursor.borrow(),
                    self.num_col.get_width(),
                )?;
                self.display_status_line()?;
            } else {
                let next_h = self.cursor.borrow().get_pos().1 + 1;
                let mut max_col = self.lines[next_h as usize].1.len() as u16;
                if max_col > 0 {
                    max_col -= 1;
                }
                let opt_col = self.cursor.borrow().get_opt_col();
                if opt_col <= max_col {
                    self.cursor.borrow_mut().set_pos((opt_col, next_h));
                } else {
                    self.cursor.borrow_mut().set_pos((max_col, next_h));
                }
                self.terminal.draw_cursor(
                    &self.lines,
                    &self.cursor.borrow(),
                    self.num_col.get_width(),
                )?;
                self.display_status_line()?;
            }
        }
        Ok(())
    }

    pub fn change_buffer(&mut self, buffer: Buffer) {
        self.buffer = Rc::new(buffer);
        self.lines = self.buffer.get_n_lines(
            self.terminal.get_size().unwrap().1 as usize - 1,
            self.scroll_offset,
        );
    }

    fn display_numcol(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.draw_numcol(&self.num_col)?;
        Ok(())
    }

    fn display_status_line(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.draw_status_line(&self.status_line)?;
        Ok(())
    }

    fn display_buffer_lines(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.draw_buffer(&self.lines, &self.num_col)?;
        Ok(())
    }

    fn display_cursor(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal
            .draw_cursor(&self.lines, &self.cursor.borrow(), self.num_col.get_width())?;
        Ok(())
    }
}

pub enum EditorMode {
    Normal,
    Insert,
    // Visual,
    // Command,
}
