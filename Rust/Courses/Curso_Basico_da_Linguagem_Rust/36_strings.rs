// Caracteres em RUST usam UTF-8 então eles aceitam outras caracteres
// String não pode ser indexado por int
fn main() {
    let s: String = String::new();
    println!("P1: s >>> {s}");

    let s: String = String::from("initial contents");
    println!("P1: s >>> {s}");

    let s: String = String::from("Rustを学んでいます");
    println!("P1: s >>> {s}");

    let s_str: &str = "conteudo inicial";
    let s: String = s_str.to_string();
    println!("P1: s >>> {s}");

    // Atualizando uma string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("P1: s >>> {s}");

    let s2: &str = "bar";
    s.push_str(s2);
    println!("P1: s >>> {s}");
    s.push('l');  // concatena um caracter
    println!("P1: s >>> {s}");

    // Concatenando Strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 é movido para s3 e s2 ainda existe
    println!("P1: s2 >>> {s2}");
    println!("P1: s3 >>> {s3}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3: String = format!("{s1}{s2}");  // Não tira a propriedade do s1
    println!("P1: s1 >>> {s1}");
    println!("P1: s2 >>> {s2}");
    println!("P1: s3 >>> {s3}");
}