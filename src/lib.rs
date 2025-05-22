use std::error::Error;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use mods::buffer::Buffer;
use mods::cursor::Cursor;
use mods::terminal::Terminal;

pub mod mods;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut term = Terminal::build()?;

    let filepath = PathBuf::from("tests/test_editor_1.txt");
    let buffer = Buffer::from_file(filepath)?;
    let buffer_size = buffer.get_size();
    let filename = buffer.get_file_name();
    let mut scroll_offset = 0;

    term.clear()?;
    let mut cursor = Cursor::new();
    term.write_status_line(&filename, &cursor)?;

    let number_col_width = (buffer.get_size().to_string().len() + 1) as u16;

    let mut lines = buffer.get_n_lines(term.get_size().unwrap().1 as usize - 1, scroll_offset);
    term.write_lines(&lines, number_col_width, buffer_size as u16)?;
    term.draw_cursor(&lines, &cursor, number_col_width)?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Right | KeyCode::Char('l') => {
                    let pos = cursor.get_pos();
                    let line_len = lines[pos.1 as usize].1.len();
                    if line_len > 0 {
                        if pos.0 < line_len as u16 - 1 {
                            cursor.move_right();
                            term.draw_cursor(&lines, &cursor, number_col_width)?;
                            term.update_status_line_cursor(&cursor)?;
                        }
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    cursor.move_left();
                    term.draw_cursor(&lines, &cursor, number_col_width)?;
                    term.update_status_line_cursor(&cursor)?;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if cursor.get_pos().1 + scroll_offset as u16 > 0 {
                        if cursor.get_pos().1 == 0 && scroll_offset > 0 {
                            scroll_offset -= 1;
                            lines = buffer.get_n_lines(lines.len(), scroll_offset);
                            term.write_lines(&lines, number_col_width, buffer_size as u16)?;
                            term.draw_cursor(&lines, &cursor, number_col_width)?;
                            term.update_status_line_cursor(&cursor)?;
                        } else {
                            let next_h = cursor.get_pos().1 - 1;
                            let mut max_col = lines[next_h as usize].1.len() as u16;
                            if max_col > 0 {
                                max_col -= 1;
                            }
                            let opt_col = cursor.get_opt_col();
                            if opt_col <= max_col {
                                cursor.set_pos((opt_col, next_h));
                            } else {
                                cursor.set_pos((max_col, next_h));
                            }
                            term.draw_cursor(&lines, &cursor, number_col_width)?;
                            term.update_status_line_cursor(&cursor)?;
                        }
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if cursor.get_pos().1 < (buffer_size - scroll_offset) as u16 - 1 {
                        if cursor.get_pos().1 == lines.len() as u16 - 1 {
                            scroll_offset += 1;
                            lines = buffer.get_n_lines(lines.len(), scroll_offset);
                            term.write_lines(&lines, number_col_width, buffer_size as u16)?;
                            term.draw_cursor(&lines, &cursor, number_col_width)?;
                            term.update_status_line_cursor(&cursor)?;
                        } else {
                            let next_h = cursor.get_pos().1 + 1;
                            let mut max_col = lines[next_h as usize].1.len() as u16;
                            if max_col > 0 {
                                max_col -= 1;
                            }
                            let opt_col = cursor.get_opt_col();
                            if opt_col <= max_col {
                                cursor.set_pos((opt_col, next_h));
                            } else {
                                cursor.set_pos((max_col, next_h));
                            }
                            term.draw_cursor(&lines, &cursor, number_col_width)?;
                            term.update_status_line_cursor(&cursor)?;
                        }
                    }
                }

                KeyCode::Enter => {}
                KeyCode::Esc => break,
                KeyCode::Char('a') => {}
                _ => {}
            }
        }
    }

    Ok(())
}

pub struct Editor {
    // buffers: Vec<Buffer>,
    terminal: &'static mut Terminal,
    buffer: Buffer,
    scroll_offset: usize,
    cursor: Cursor,
    mode: EditorMode,
    lines: Vec<(u16, String)>,
    // num_col: NumColumn,
    // status_line: StatusLine
}

impl Editor {
    pub fn new(terminal: &'static mut Terminal) -> Self {
        Self {
            terminal,
            buffer: Buffer::new(),
            scroll_offset: 0,
            cursor: Cursor::new(),
            mode: EditorMode::Normal,
            lines: vec![],
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.draw_cursor(&self.lines, &self.cursor, 3)?;
        todo!();
    }
}

enum EditorMode {
    Normal,
    // Insert,
    // Visual,
    // Command,
}
