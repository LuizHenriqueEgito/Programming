fn outra_funcao() {
    println!("Outra função");
}

fn outra_funcao_com_parametro(x: i32) {
    println!("Outra função recebeu: {x}.");
}

fn soma(x: i32, y: i32) -> i32 {
    x + y // não colocar ; automaticamente esse valor é o retorno da função
}

fn soma_return(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    println!("Iniciando");
    outra_funcao();
    // não existem parametros nomeados no Rust
    outra_funcao_com_parametro(1);
    let xy = soma(2 ,2);
    let yx = soma_return(2, 4);
    println!("Primeira soma: {xy}");
    println!("Segunda soma: {yx}");
}