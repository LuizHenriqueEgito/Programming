/*
Isso só irá funcionar quando você utilizar Cargo e o nome da pasta for hello
hello
|__ Cargo.toml
|__ src
    |__ lib.rs
    |__ main.rs  // the hello binary
*/

use hello::greet;

fn main() {
    hello::greet();
}