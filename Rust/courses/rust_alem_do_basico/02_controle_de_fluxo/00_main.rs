fn main() {
    let number: i32 = 3;
    condicao(number);

    // semelhante a um operador ternario
    let cond: bool = false;
    let resultado: &str = if cond {
        "Condição verdadeira"
    } else {
        "Condição falsa"
    };
    println!("{}", resultado);

    // loop - repita (é como um while true)
    loop_infinito();

    // while - enquanto
    loop_while();

    // for
    loop_for();

    // match
    match_case();
}

// match
fn match_case() {
    let estacao: &str = "verao";
    match estacao {
        "primavera" => {
            println!("É primavera");
        },
        "verao" => {
            println!("É verao");
        },
        "outono" => {
            println!("É outono");
        },
        "inverno" => {
            println!("É inverno");
        }
        _ => {
            println!("Desconhecido.")
        }
    }
}

// loop
fn loop_infinito() {
    let mut c: i32 = 0;
    loop {
        println!("Loop: {}", c + 1);
        c += 1;
        if c == 10 {
            break;
        }
    }
}

// while
fn loop_while() {
    let mut c: i32 = 0;
    while c < 5 {
        println!("While: {}", c + 1);
        c += 1;
    }
}

// for
fn loop_for() {
    for i in 0..10 {
        println!("For: {}", i);
    }
    for i in 0..=10 {
        println!("For (=): {}", i);
    }
}

// if else
fn condicao(number: i32) {
    if number > 0 {
        println!("O número {} é positivo.", number);
    } else if number == 0 {
        println!("O número é zero: {}", number);
    } else {
        println!("O número {} é negativo", number);
    }
}