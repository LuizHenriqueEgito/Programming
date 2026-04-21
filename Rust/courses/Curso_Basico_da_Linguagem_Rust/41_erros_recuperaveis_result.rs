/*
File::open retorna Reesult<T, E>
T é std::fs::File
E é std::io::Error
*/

use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;

fn main() {
    let _greeting_file_result: Result<File, Error> = File::open("41_hello.txt");
    let _greeting_file: File = match _greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("41_hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problema ao criar arquivo: {:?}", e),
            },
            other_error => {
                panic!("Problema ao abrir o arquivo: {:?}", other_error);
            }
        }
    };

}