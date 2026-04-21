use std::io;


fn convert_to_int(input: &str) -> i32 {
    input
        .trim()
        .parse::<i32>()
        .expect("Erro ao converter para número")
}

fn main() {
    let mut soma = 0;
    let mut valor_entrada = String::new();
    io::stdin().read_line(&mut valor_entrada).expect("Erro ao ler o valor...");
    let mut valor_i32 = convert_to_int(&valor_entrada);

    while valor_i32 != 0 {
        let0 r = valor_i32 % 10;
        soma += r;
        valor_i32 = valor_i32 / 10;
    }
    println!("O valor da doma dos digitos {}", soma)
}