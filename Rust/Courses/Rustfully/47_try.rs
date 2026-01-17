use std::fs::{self, File};
use std::io::{self, Read}; // {self} é a mesma coisa que uma linha só com use::io


fn get_data() -> Result<>String, io::Error> {
    let mut data_file = File::open("data.txt")?;
    // Colocar ? ao final já faz tudo isso para nós, é um açucar sintatico
    // let mut data_file = match data_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut data = String::new();
    data_file.read_to_string(&mut data)?;
    // Agora depois do ? da linha acima a parte de baixo não será mais precisa
    // match data_file.read_to_string(&mut data) {
    //     Ok(_) = Ok(data),
    //     Err(e) => Err(e),
    // }
    Ok(data)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let data = get_data();
    let c = last_char_of_first_line("Is tha you Bob?");
    let n = last_char_of_first_line("");
}