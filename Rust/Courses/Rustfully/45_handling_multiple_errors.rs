/*
Rust implementa:
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/
use std::fs::File;

fn main() {
    let path = "secret.txt";
    let txt = File::open(path);

    match txt {
        Ok(mut file) => {
            let mut contents: String = String::new();
            let text = file.read_to_string(&mut contents);
            println!("File loaded: {}", contents)
        }
        Err(error) => panic!("Problem opening the file: {error:?}"),
    }
}