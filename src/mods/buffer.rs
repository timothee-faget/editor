use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

pub struct TextBuffer {
    data: Vec<char>,
    cursor: Cursor,
}

impl TextBuffer {
    pub fn new(data: Vec<char>) -> Self {
        Self {
            data,
            cursor: Cursor::new(),
        }
    }

    pub fn from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        let mut buffer = Self::new(vec![]);

        for c in content.chars() {
            buffer.push(c);
        }

        Ok(buffer)
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for ch in &self.data {
            string.push(*ch);
        }

        string
    }

    // Return lines
    pub fn to_lines(&self) -> Vec<String> {
        self.to_string().lines().map(|l| String::from(l)).collect()
    }

    /// Add char to buffer
    pub fn push(&mut self, ch: char) {
        self.data.push(ch);
    }

    /// Delete a buffer char
    pub fn delete(&mut self, index: usize) {
        if index < self.data.len() {
            self.data.remove(index);
        } else {
            eprintln!("Removing index is greater than buffer lenght");
        }
    }

    /// Delete buffer chars from range.0 to range.1 - 1
    pub fn delete_range(&mut self, rng: (usize, usize)) {
        for _ in 0..(rng.1 - rng.0) {
            self.delete(rng.0);
        }
    }

    /// Return number of lines in buffer
    pub fn lines(&self) -> u32 {
        self.data.iter().filter(|&c| *c == '\n').count() as u32 + 1
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) {
        match direction {
            CursorDirection::Up => self.cursor.y -= 1,
            CursorDirection::Right => self.cursor.x += 1,
            CursorDirection::Down => self.cursor.y += 1,
            CursorDirection::Left => self.cursor.x -= 1,
        }
    }

    pub fn cursor_position(&self) -> usize {
         e
        todo!();
    }
}

pub struct Cursor {
    x: u32,
    y: u32,
    prev_y: u32,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            prev_y: 0,
        }
    }

    pub fn set_x(&mut self, new_x: u32) {
        self.x = new_x;
    }

    pub fn set_y(&mut self, new_y: u32) {
        self.y = new_y;
    }
}

pub enum CursorDirection {
    Up,
    Right,
    Down,
    Left,
}

#[cfg(test)]
mod buffer_tests {
    use std::vec;

    use super::TextBuffer;

    #[test]
    fn delete() {
        let mut buffer = TextBuffer::new(vec!['a', 'b', 'c', 'd', 'e']);

        buffer.delete(4);
        assert_eq!(buffer.to_string(), String::from("abcd"));

        buffer.delete(0);
        assert_eq!(buffer.to_string(), String::from("bcd"));

        buffer.delete(1);
        assert_eq!(buffer.to_string(), String::from("bd"));

        buffer.delete(100);
        assert_eq!(buffer.to_string(), String::from("bd"));
    }

    #[test]
    fn delete_range() {
        let mut buffer = TextBuffer::new(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']);

        buffer.delete_range((3, 6));

        assert_eq!(buffer.to_string(), String::from("abcghi"));
    }

    #[test]
    fn lines() {
        let buffer = TextBuffer::new(vec!['a', 'b', '\n', 'c', '\n', 'd']);
        assert_eq!(buffer.lines(), 3);

        let buffer = TextBuffer::new(vec!['a', 'b', 'c', '\n', 'd']);
        assert_eq!(buffer.lines(), 2);

        let buffer = TextBuffer::new(vec!['a', 'b']);
        assert_eq!(buffer.lines(), 1);
    }

    #[test]
    fn to_lines() {
        let buffer = TextBuffer::new(vec!['a', 'a', '\n', 'b']);
        let result = buffer.to_lines();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], String::from("aa"));
        assert_eq!(result[1], String::from("b"));
    }
}
