// use editor::TextBuffer;
use editor::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Erreur {}", e);
    }
}
