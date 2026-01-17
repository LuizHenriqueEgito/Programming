/*
Uma macro possui a seguinte sintaxe:
macro_rules! macro_name {
    (pattern) => {
        // generate code
    }-
}

- `:expr` - Matches any expression (like `5`, `x + y`, `foo()`)
- `:ident` - Matches an identifier (like `x`, `my_function`, `MyStruct`)
- `:ty` - Matches a type (like `i32`, `String`, `Vec<u8>`)
- `:path` - Matches a path (like `std::collections::HashMap`)
- `:literal` - Matches a literal (like `42`, `"Hello"`, `true`)
- `:block` - Matches a block of code (like `{ x + y}`)
- `:stmt` - Matches a statement (like `let x = 5;`)
- `:pat` - MAtches a pattern (like `Some(x)`, `_`)

`:expr` e `:ident` são as mais usadas.
*/
use std::collections::HashMap;


macro_rules! doublçe {  // duplica um numero
    ($x:expr) => {  // $x -> é uma metavariavel que captura o valor correspondente
        $x * 2
    };
}

macro_rules! create_var {
    ($name:ident) => {
        let $name = 42; 
    };
}

macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}", $name);
    };
    ($name:expr, $greeting:expr) => {
        println!("{}, {}!", $greeting, $name);
    };
}

macro_rules! hashmap {
    ($($key:expr => $value:expr), * $(,)?) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

fn main() {
    let result = double!(5 + 10);  // 5 + 10 é a expressão: expr
    println!("Double!(%) = {result}");

    create_var!(my_number);// é a mesma coisa que fazer isso: let my_number = 42;
    println!("my_number = {my_number}");

    greet!("Alyce");
    greet!("Bob", "Good morning");

    let map = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };

    println!("{:?}", map);
    println!("map['one'] = {}", map["one"]);
}

