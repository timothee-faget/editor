use editor::TextBuffer;

fn main() {
    let mut buffer = TextBuffer::new(vec!['a', 'b', 'c', '\n', 'b']);

    println!("{}", buffer.to_string());
    buffer.push('c');
    println!("{}", buffer.to_string());
}
