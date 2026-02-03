// usamos snake_case em Rust

// Precisamos tipar a nossa função
// Não é possivel ter parametros default devem ser passados (podemos criar structs para contornar isso, da para fazer com uma macro também)
// Em Rust todos os argumntos são posicionais (é melhor ter um struct) 
fn say_hello(name: &str) {
    // corpo da função
    println!("Hello {}", name);
}

// Expression cria um valor 5 + 5 cria um valor é uma expression
// por isso em funçções as vezes não precisamos colocar result pois
// o que fazemos jé é uma expression (não colocamos ;)
fn add_numbers(x: i32 , y: i32) -> i32 {
    // return x + y;
    if x == 0 {
        return y;  // earling return aqui precisamos de return e ;
    };
    x + y
}

fn main() {
    say_hello("Luiz");  // chama a função
    say_hello("Maria");

    let y = {
        say_hello("Carol");
        let x = 5;
        99  // igual ao retorno de uma função não colocamos ;
    };
    println!("{:?}", y);
    let res = add_numbers(8, 9);
    println!("{res}");

    let input: Vec<i32> = "56 65 58 48 59 56 87 23";
    let result = input
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())  // parse::<i32>()  --> fala o tipo em que ele deve fazer o parse
        .map(|n| n * 2)
        .collect();

}