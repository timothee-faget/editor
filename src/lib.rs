use std::error::Error;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use mods::buffer::Buffer;
use mods::cursor::Cursor;
use mods::numcol::NumColumn;
use mods::statusline::StatusLine;
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

pub fn run_editor() -> Result<(), Box<dyn Error>> {
    let mut term = Terminal::build()?;

    let mut editor = Editor::build(PathBuf::from("tests/test_editor_1.txt"), &mut term)?;
    editor.run()?;

    Ok(())
}

pub struct Editor<'a> {
    terminal: &'a mut Terminal,
    buffer: Buffer,
    scroll_offset: usize,
    cursor: Cursor,
    mode: EditorMode,
    lines: Vec<(u16, String)>,
    num_col: NumColumn,
    status_line: StatusLine,
}

impl<'a> Editor<'a> {
    pub fn new(terminal: &'a mut Terminal) -> Self {
        Self {
            terminal,
            buffer: Buffer::new(),
            scroll_offset: 0,
            cursor: Cursor::new(),
            mode: EditorMode::Normal,
            lines: vec![],
            num_col: NumColumn::new(),
            status_line: StatusLine::new(),
        }
    }

    pub fn build(filepath: PathBuf, terminal: &'a mut Terminal) -> Result<Self, Box<dyn Error>> {
        let buffer = Buffer::from_file(filepath)?;
        let buffer_size = buffer.get_size() as u16;
        let term_size = terminal.get_size().unwrap().1;
        let lines = buffer.get_n_lines(term_size as usize, 0);
        Ok(Self {
            terminal,
            buffer,
            scroll_offset: 0,
            cursor: Cursor::new(),
            mode: EditorMode::Normal,
            lines,
            num_col: NumColumn::build(buffer_size, 0, term_size - 1),
            status_line: StatusLine::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // Initialisation
        self.display_numcol()?;

        // Main loop
        loop {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Right | KeyCode::Char('l') => {}
                    KeyCode::Left | KeyCode::Char('h') => {}
                    KeyCode::Up | KeyCode::Char('k') => {}
                    KeyCode::Down | KeyCode::Char('j') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn change_buffer(&mut self, buffer: Buffer) {
        self.buffer = buffer;
        self.lines = self.buffer.get_n_lines(
            self.terminal.get_size().unwrap().1 as usize - 1,
            self.scroll_offset,
        );
    }

    fn display_numcol(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.draw_numcol(&self.num_col)?;
        Ok(())
    }

    fn display_buffer_lines(&self) {}
}

enum EditorMode {
    Normal,
    // Insert,
    // Visual,
    // Command,
}
