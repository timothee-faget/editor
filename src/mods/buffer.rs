use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct Buffer {
    filepath: PathBuf,
    data: Vec<String>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            filepath: PathBuf::new(),
            data: vec![String::new()],
        }
    }

    pub fn from_vec(vec: Vec<String>) -> Self {
        Self {
            filepath: PathBuf::new(),
            data: vec,
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        Ok(Self {
            filepath: path,
            data: reader.lines().collect::<Result<Vec<String>, _>>()?,
        })
    }

    pub fn get_file_name(&self) -> String {
        if let Some(name) = self.filepath.file_name() {
            String::from(name.to_str().unwrap()) // WARNING
        } else {
            String::new()
        }
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_n_lines(&self, n: usize, scroll_offset: usize) -> Vec<(u16, String)> {
        let mut lines = Vec::new();
        for i in 0..n {
            lines.push((
                (i + scroll_offset) as u16,
                self.data
                    .get(i + scroll_offset)
                    .map_or(String::new(), |v| v.to_string()),
            ));
        }
        lines
    }

    pub fn insert(&mut self, c: char, position: BufferPosition) {
        self.data[position.y].insert(position.x, c);
    }

    pub fn delete(&mut self, position: BufferPosition) {
        self.data[position.y].remove(position.x);
    }

    pub fn split_line(&mut self, position: BufferPosition) {
        let binding = self.data[position.y].clone();
        let splited_lines = binding.split_at(position.x);
        self.data.remove(position.y);
        self.data.insert(position.y, String::from(splited_lines.0));
        self.data
            .insert(position.y + 1, String::from(splited_lines.1));
    }

    pub fn delete_range(&mut self, selection: BufferSelection) {
        if selection.start.y == selection.end.y {
            for _ in 0..selection.end.x - selection.start.x + 1 {
                self.delete(selection.start);
            }
        } else {
            for i in (selection.start.y + 1)..(selection.end.y) {
                self.data.remove(i);
            }
            while self.data[selection.start.y]
                .get(selection.start.x..selection.start.x + 1)
                .is_some()
            {
                self.data[selection.start.y].remove(selection.start.x);
            }
            for _ in 0..selection.end.x + 1 {
                self.data[selection.start.y + 1].remove(0);
            }
            let end_line = self.data[selection.start.y + 1].clone();
            self.data.remove(selection.start.y + 1);
            self.data[selection.start.y].push_str(&end_line);
        }
    }
}

#[derive(Clone, Copy)]
pub struct BufferPosition {
    x: usize,
    y: usize,
}

impl BufferPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct BufferSelection {
    start: BufferPosition,
    end: BufferPosition,
}

impl BufferSelection {
    pub fn new(start: BufferPosition, end: BufferPosition) -> Self {
        Self { start, end }
    }
}

#[cfg(test)]
mod tests_buffer {
    use std::path::PathBuf;

    use super::{Buffer, BufferPosition, BufferSelection};

    #[test]
    fn insert() {
        let mut buffer = Buffer::from_vec(vec![
            String::from("Salut les amis"),      // 14
            String::from("Comment allez vous?"), // 19
        ]);

        buffer.insert('a', BufferPosition::new(3, 0));
        assert_eq!(buffer.data[0], String::from("Salaut les amis"));

        buffer.insert('b', BufferPosition::new(0, 1));
        assert_eq!(buffer.data[1], String::from("bComment allez vous?"));
    }

    #[test]
    fn delete() {
        let mut buffer = Buffer::from_vec(vec![
            String::from("Salut les amis"),      // 14
            String::from("Comment allez vous?"), // 19
        ]);

        buffer.delete(BufferPosition::new(0, 0));
        assert_eq!(buffer.data[0], String::from("alut les amis"));

        buffer.delete(BufferPosition::new(3, 1));
        assert_eq!(buffer.data[1], String::from("Coment allez vous?"));
    }

    #[test]
    fn split_line() {
        let mut buffer = Buffer::from_vec(vec![String::from("Bonjour les amis")]);

        buffer.split_line(BufferPosition::new(7, 0));
        assert_eq!(buffer.data.len(), 2);
        assert_eq!(
            buffer.data,
            vec![String::from("Bonjour"), String::from(" les amis")]
        );
    }

    #[test]
    fn delete_range() {
        let mut buffer = Buffer::from_vec(vec![
            String::from("Bonjour les amis"),
            String::from("Comment allez vous?"),
        ]);

        let selection = BufferSelection::new(BufferPosition::new(0, 0), BufferPosition::new(6, 0));
        buffer.delete_range(selection);
        assert_eq!(buffer.data[0], String::from(" les amis"));

        let selection = BufferSelection::new(BufferPosition::new(8, 0), BufferPosition::new(2, 1));
        buffer.delete_range(selection);
        assert_eq!(buffer.data.len(), 1);
        assert_eq!(buffer.data[0], String::from(" les amiment allez vous?"));

        let mut buffer = Buffer::from_vec(vec![
            String::from("Bonjour les amis"),
            String::from("Comment allez vous?"),
        ]);

        let selection = BufferSelection::new(BufferPosition::new(0, 0), BufferPosition::new(18, 1));
        buffer.delete_range(selection);
        assert_eq!(buffer.data, vec![String::from("")]);

        let mut buffer = Buffer::from_vec(vec![
            String::from("Bonjour les amis"),
            String::from("Je suis là pour vous aider"),
            String::from("Comment allez vous?"),
        ]);

        let selection = BufferSelection::new(BufferPosition::new(0, 0), BufferPosition::new(18, 2));
        buffer.delete_range(selection);
        assert_eq!(buffer.data, vec![String::from("")]);

        let mut buffer = Buffer::from_vec(vec![
            String::from("Bonjour les amis"),
            String::from("Bonjour les amis"),
            String::from("Comment allez vous?"),
        ]);

        let selection = BufferSelection::new(BufferPosition::new(6, 0), BufferPosition::new(10, 2));
        buffer.delete_range(selection);
        assert_eq!(buffer.data, vec![String::from("Bonjouez vous?")]);
    }

    #[test]
    fn from_file() {
        let path = PathBuf::from("tests/from_file_test.txt");

        let buffer = Buffer::from_file(path);

        assert!(buffer.is_ok());

        let buffer = buffer.unwrap();

        assert_eq!(buffer.data[0], String::from("Bonjour les amis"));
        assert_eq!(buffer.data[1], String::from("Comment allez vous?"));
    }

    #[test]
    fn get_size() {
        let buffer = Buffer::from_vec(vec![
            String::from("Bonjour les amis"),
            String::from("Bonjour les amis"),
            String::from("Comment allez vous?"),
        ]);

        assert_eq!(buffer.get_size(), 3);
    }
}
