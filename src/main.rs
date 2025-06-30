// use editor::TextBuffer;
// use editor::run;

fn main() {
    if let Err(e) = editor::run_editor() {
        println!("Erreur {}", e);
    }
}
