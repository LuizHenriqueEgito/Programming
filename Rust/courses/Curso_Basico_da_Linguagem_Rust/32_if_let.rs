// if let: se o casamento funcionar ele executa o código se não ele não faz nada
/*
if let é uma forma curta de match quando você só se importa com um padrão.
let x = Some(10);

if let Some(valor) = x {
    println!("Valor é {}", valor);
}
“Se x for Some(valor), execute o bloco.”
Se for None → nada acontece.

Usando com else 
if let Some(valor) = x {
    println!("Tem valor: {}", valor);
} else {
    println!("Não tem valor");  // Agora valor existe
}
if let é um match preguiçoso e elegante.
*/

fn main() {
    let config_max: Option<i32> = Some(3);
    // usando match normal
    match config_max {
        Some(max) => println!("Match, The maximum is configured to be {}", max),
        _ => (),
    }

    // usando if let
    if let Some(max) = config_max {
        println!("Match, The maximum is configured to be {}", max);
    }
}