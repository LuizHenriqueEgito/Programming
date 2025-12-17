/* 
Copy:
    - é uma copia burra bit a bit
    - é mais rápido
    - acionado pela atribuição '='
    - tendo copy automaticamente tem Clone

Clone:
    - é uma duplicação inteligente de todos os componentes e subcomponentes
    - é mais lento que o Copy
    - acionado pelo método 'clone()'
    - pode ter Clone mas não ter o Copy
*/

fn main() {
    println!("Semântica Copy\n");
    let x: i32 = 5;
    let y: i32 = x;  // semantica copy
    println!("x = {x}");
    println!("y = {y}");

    // Semântica copy está disponível quando o tipo implementa o `trait` Copy
    // Int, bool, float, Char, Tuplas e Arrays
    // String não

    println!("Semântica Move\n");
    let s1: String = String::from("hello");
    // Valor 'Tipo String' é movido para s2
    let s2: String = s1;  // s1 não é mais válida!

    println!("s2 = {s2}");
    // println!("s1 = {s1}");  -> isso dá erro

    // Ainda é possivel fazer um clone
    let s3: String = s2.clone();
    println!("s3 = {s3}");
    println!("s2 = {s2}");
}