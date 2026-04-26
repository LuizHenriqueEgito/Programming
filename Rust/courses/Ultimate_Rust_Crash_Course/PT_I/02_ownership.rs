/*
1. Each value has an owner
2. Only one owner
3. Values gets dropped if its owner goes out of scope

Stack            Heap
ptr: -----------> "a | b | c"
len: 3
capacity: 3
*/

fn main() {
    let s1 = String::from("abc");
    let s2 = s1;  // s1 para de existir
    println!("{}", s2);

    // podemos contornar isso usando
    let s1 = String::from("ABC");
    let s2 = s1.clone();  // cria outro "objeto" na Heap
    println!("{}, {}", s1, s2);

}