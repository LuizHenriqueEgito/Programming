// Tipos primitivos são tipos básicos
/*
Tipos escalares & tipos compostos

# ESCALARES (scalar types)
- Representa um único valor contido dentro de uma escala conhecida
- Permitem a comparação direta entre valores
## Tipos
- Inteiros (integer): 5
- Flutuante (floating point): 42.1
- Booleano (bool): true, false
- Caractere (char): a

# COMPOSTOS (Compound Types)
- Servem para agregar multiplos valores
## Tipos
- Tupla (tuple): (5, true, 42.1, 'a')
- Matriz (array) [1, 2, 3, 4, 5, 6] 
*/
fn main() {
    // SCALAR
    // int
    let x1: u8 = 5;
    let x2 = 5_u8;  // o rust também aceita assim (pouco usado)
    let x3 = 199_456_890;
    let h = 0xff;  // hexadecimal
    let o = 0x77; // octal
    let b = 0b1111_0000; // binario
    let by = b'A';

    // float
    let x4: f64 = 42.1;
    
    // bool
    let x5: bool = true;
    let x6 = false

    // char
    let word: char = 'a'  // neste caso aspas simples

    // COMPOUND
    // tuplas
    let x7: (i32, i32, i32) = (1, 2, 3);  // tem tamanho fixo
    println!("{:?}", x7);  // só printa no modo debugg
    println!("{:?}", x7.0);  // pega o primeiro elemento
    let (a, b, c) = x7;  // faz o desempacotamento

    let mut numbers = (1, 2, 3);  // podemos substituir
    numbers.0 = 11;
    // podemos mudar a tupla inteira
    numbers = (4, 5 ,6);
    println!("{:?}", numbers);

    // arrays (não permite tipos diferentes como uma tupla)
    let x8: [i32; 3] = [1, 2, 3];
    println!("{:?}", x8[0]);

    let mut x9: [f64, 3] = [1.1, 2.2, 3.3];
    x9[1] = 4.4;
    println!("{:?}", x9)

    // slices
    println!("{:?}", &x9[1..2])
}