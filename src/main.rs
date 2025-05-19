use editor::run_editor;

fn main() {
    println!("Hello World");
    if let Err(e) = run_editor() {
        eprintln!("Erreur dans l'éditeur : {}", e);
    } else {
        println!("Tout s'est bien passé, sur le papier du moins")
    }
}
