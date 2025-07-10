// TODO  Changer pour des HashMaps

use crossterm::style::Color;
use std::{cell::RefCell, rc::Rc};

use crate::EditorMode;

use super::{buffer::Buffer, cursor::Cursor, terminal::CharStyle};

pub struct StatusLine {
    mode: Rc<EditorMode>,
    cursor: Rc<RefCell<Cursor>>,
    buffer: Rc<Buffer>,
    scroll_offset: Rc<usize>,
}

impl StatusLine {
    pub fn new(
        mode: Rc<EditorMode>,
        cursor: Rc<RefCell<Cursor>>,
        buffer: Rc<Buffer>,
        scroll_offset: Rc<usize>,
    ) -> Self {
        Self {
            mode,
            cursor,
            buffer,
            scroll_offset,
        }
    }

    pub fn get_blocks(&self) -> Vec<String> {
        vec![
            self.get_mode_block(),
            self.get_filename_block(),
            self.get_cursor_block(),
        ]
    }

    fn get_mode_block(&self) -> String {
        match *self.mode {
            EditorMode::Normal => String::from(" NORMAL "),
            EditorMode::Insert => String::from(" INSERT "),
        }
    }

    fn get_filename_block(&self) -> String {
        format!(" {} ", self.buffer.get_file_name())
    }

    fn get_cursor_block(&self) -> String {
        let (x, y) = self.cursor.borrow().get_pos();
        format!(" {}:{} ", x, y + self.scroll_offset.to_be() as u16)
    }
}

pub struct StatusLineSide {
    modules: Vec<StatusLineModule>,
}

impl StatusLineSide {
    pub fn new() -> Self {
        Self { modules: vec![] }
    }

    pub fn push(&mut self, module: StatusLineModule) {
        if self.modules.len() < 3 {
            self.modules.push(module);
        }
    }

    pub fn len(&self) -> usize {
        self.modules.iter().map(|m| m.len()).sum()
    }

    pub fn get_blocks(&self) -> Vec<String> {
        self.modules.iter().map(|m| m.text()).collect()
    }

    pub fn set_module_text(&mut self, module: &String, text: &String) -> Option<()> {
        if let Some(m) = self.modules.iter_mut().find(|m| m.name == *module) {
            m.text = text.to_string();
            Some(())
        } else {
            None
        }
    }
}

pub struct StatusLineModule {
    name: String,
    text: String,
    style: CharStyle,
}

impl StatusLineModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            text: String::new(),
            style: CharStyle::new(Color::White, Color::Blue),
        }
    }

    pub fn len(&self) -> usize {
        self.text.len() + 2
    }

    pub fn text(&self) -> String {
        format!(" {} ", self.text)
    }

    pub fn style(&self) -> CharStyle {
        self.style
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text
    }
}
