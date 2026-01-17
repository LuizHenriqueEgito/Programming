use std::fs::File;
use std::io::{self, Read}; // {self} é a mesma coisa que uma linha só com use::io

fn get_data() -> Result<>String, io::Error> {
    let data_file_result = File::open("data.txt");
    let mut data_file = match data_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut data = String::new();
    match data_file.read_to_string(&mut data) {
        Ok(_) = Ok(data),
        Err(e) => Err(e),
    }
}

fn main() {
    let data = get_data();
}