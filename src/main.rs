// use editor::TextBuffer;
use editor::run;

fn main() {
    if let Err(e) = run() {
        println!("Erreur {}", e);
    }
}
