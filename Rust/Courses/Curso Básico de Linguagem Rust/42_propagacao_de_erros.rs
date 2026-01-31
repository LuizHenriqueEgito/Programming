/*
Operador `?`, propaga Err se fo o caso, ou retorna o conteúdo de Ok
Operador `?`, pode ser usado apenas quando o tipo do retorno da função for compatível com Err(E) recebido por `?`
Operador `?`, pode converter o tipo de erro no retorno, mas requer `Traits`

`?` Também funciona com Option<T, E>
*/
use std::fs;
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file: File = File::open("42_username.txt")?;
    let mut username: String = String::new();
    let n_bytes: usize = username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn versao_compacta() -> Result<String, io::Error> {
    let mut username: String = String::new();
    File::open("42_username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Fazer isso é tão normal que a própria lib já englobou
fn read_username() -> Result<String, io::Error> {
    fs::read_to_string("42_username.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text
    .lines()
    .next()?
    .chars()
    .last()
}

fn main() {
    let username = match read_username_from_file() {
        Ok(username) => username,
        Err(_) => "guest".to_string(),
    };
    println!("Username é {}", username);

    let username = match versao_compacta() {
        Ok(username) => username,
        Err(_) => "guest".to_string(),
    };
    println!("Username é {}", username);

    let last: Option<char> = last_char_of_first_line(&username);
    println!("caracter: {}", last.unwrap());
}