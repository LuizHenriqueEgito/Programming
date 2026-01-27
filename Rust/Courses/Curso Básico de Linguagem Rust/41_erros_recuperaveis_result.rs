/*
File::open retorna Reesult<T, E>
T é std::fs::File
E é std::io::Error
*/

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result: Result<File, Error> = File::open("hello.txt");

}