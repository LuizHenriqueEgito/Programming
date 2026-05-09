/*
STACK:
LIFO - last in first out
O que vai para a Stack:
- escalares
- inteiros
- floats
- caracteres
- booleanos
- tuplas

HEAP:
A alocação de memotia na HEAP é feita através de ponteiros, que são referencias a locais na HEAP
- vetores
- strings
- structs
*/

fn main() {
    // Estão na Stack não temos problema de ownership
    let x: i32 = 1;
    let y: i32 = x;
    println!("x {} | y {}", x, y);

    let name: String = String::from("String na HEAP");
    let name_2 = name;
    println!("Agora eu só tenho o nome_2: {}", name_2);

    let name: String = String::from("Nova String");
    exemplo_borrowing(&name);
    println!("Minhas var ainda existe: {}", name);
}

fn exemplo_borrowing(string: &String) {
    println!("---");
    println!("Eu emprestei minha var mas ela ainda existe!");
    println!("Var emprestada:");
    println!("{}", string);
    println!("---");
}