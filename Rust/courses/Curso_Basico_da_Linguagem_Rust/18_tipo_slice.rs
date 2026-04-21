/*
Slices são referencias para uma sequencia continua de elementos
Um slice é um tipo de referencia
*/
fn main() {
    let s: String = String::from("hello world");
    let s1: &str = &s[0..5]; // msm coisa &s[..5];
    let s2: &str = &s[6..11];
    let s3: &str = &s[..2];
    let s4: &str = &s[3..];

    let slit: &str = "Hello, world";
    println!("s1: {}, s2: {}, s3: {}, s4: {}, slit: {}", s1, s2, s3, s4, slit);

    let s: String = String::from("Alo mundo cruel");
    let word: &str = get_first_word(&s);
    println!("The first word is: {}", word);

    println!("Todos os exemplos");
    let my_string: String = String::from("Alo Mundo Cruel");
    let my_string_literal: &str = "Hello Cruel World";
    
    // Parametro poed ser um slice
    println!("Parametro poed ser um slice");
    let word = get_first_word(&my_string[0..6]);
    println!("{word}");

    let word = get_first_word(&my_string[..]);
    println!("{word}\n");

    // Parametro pode ser referencia para String
    println!("PParametro pode ser referencia para String");
    let word = get_first_word(&my_string);
    println!("{word}\n");

    // Parametro pode ser slice de 'String Literal'
    println!("Parametro pode ser slice de 'String Literal'");
    let word = get_first_word(&my_string_literal[0..6]);
    println!("{word}\n");

    // Parametro pode ser um 'String Literal', pois ele equivale a um '&str'
    println!("Parametro pode ser um 'String Literal', pois ele equivale a um '&str'");
    let word = get_first_word(my_string_literal);
    println!("{word}\n");

    // my_string_literal ainda existe
    println!("{my_string_literal}")
}

fn get_first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]  // string fatiada
}