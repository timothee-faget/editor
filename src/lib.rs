pub mod mods;

// pub struct TextBuffer {
//     data: Vec<char>,
// }
//
// impl TextBuffer {
//     pub fn new(data: Vec<char>) -> Self {
//         Self { data }
//     }
//
//     pub fn to_string(&self) -> String {
//         let mut string = String::new();
//         for ch in &self.data {
//             string.push(*ch);
//         }
//
//         string
//     }
//
//     pub fn push(&mut self, ch: char) {
//         self.data.push(ch);
//     }
//
//     pub fn delete(&mut self, index: usize) {
//         if index < self.data.len() {
//             self.data.remove(index);
//         } else {
//             eprintln!("Removing index is greater than buffer lenght");
//         }
//     }
//
//     pub fn delete_range(&mut self, rng: (usize, usize)) {
//         for _ in 0..(rng.1 - rng.0) {
//             self.delete(rng.0);
//         }
//     }
//
//     pub fn lines(&self) -> u32 {
//         self.data.iter().filter(|&c| *c == '\n').count() as u32 + 1
//     }
// }
//
// // struct CursorPosition {
// //     line: u32,
// //     col: u32,
// // }
// //
// // pub struct Cursor {
// //     position: CursorPosition,
// // }
//
// #[cfg(test)]
// mod buffer_tests {
//     use std::vec;
//
//     use crate::TextBuffer;
//
//     #[test]
//     fn delete() {
//         let mut buffer = TextBuffer::new(vec!['a', 'b', 'c', 'd', 'e']);
//
//         buffer.delete(4);
//         assert_eq!(buffer.to_string(), String::from("abcd"));
//
//         buffer.delete(0);
//         assert_eq!(buffer.to_string(), String::from("bcd"));
//
//         buffer.delete(100);
//         assert_eq!(buffer.to_string(), String::from("bcd"));
//     }
//
//     #[test]
//     fn delete_range() {
//         let mut buffer = TextBuffer::new(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']);
//
//         buffer.delete_range((3, 6));
//
//         assert_eq!(buffer.to_string(), String::from("abcghi"));
//     }
//
//     #[test]
//     fn lines() {
//         let buffer = TextBuffer::new(vec!['a', 'b', '\n', 'c', '\n', 'd']);
//         assert_eq!(buffer.lines(), 3);
//
//         let buffer = TextBuffer::new(vec!['a', 'b', 'c', '\n', 'd']);
//         assert_eq!(buffer.lines(), 2);
//
//         let buffer = TextBuffer::new(vec!['a', 'b']);
//         assert_eq!(buffer.lines(), 1);
//     }
// }
