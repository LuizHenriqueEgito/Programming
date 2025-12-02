fn main() {
    // IF ELSE
    println!("Usando IF ELSE");
    let number: i32 = 3;

    if number < 5 {
        println!("condição é verdadeira")
    } else {
        println!("condição é falsa")
    }

    // Cascata de ifs
    if number % 4 == 0 {
        println!("O número é divisivel por 4");
    } else if  number % 3 == 0 {
        println!("O número é divisível por 3");
    } else if number % 2 == 0 {
        println!("O número é divisivel por 2");
    } else {
        println!("O número não é divisivel por 4, 3 ou 2")
    }

    let other_number: i32 = if number == 0 { 0 } else { 99 };
    println!("O valor do other_numer é: {other_number}");

    let other_number2: i32 = if number == 3 { 0 } else { 99 };
    println!("O valor do other_numer é: {other_number2}");
    println!("==========================================");

    // WHILE
    let mut number: i32 = 3;
    println!("\n Usando WHILE");
    while number != 0 {
        println!("while {number}");
        number -= 1;
    }

    // FOR
    let aaa: [i32; 5] = [1, 2, 3, 4, 5];
    for elemento in aaa {
        println!("O valor é: {elemento}");
    }
    // range exclusivo
    for number in 1..4 {
        println!("com = {number}");
    }
    println!("---");
    // range inclusivo
    for number in 1..=4 {
        println!("com = {number}");
    }
    println!("---");
    // range reverso
    for number in (1..4).rev() {
        println!("com = {number}");
    }
    println!("---");
    // range maior
    for i in 0..=10 {
        if i % 2 == 0 {
            println!("{i}: Par");
        } else {
            println!("{i}: Impar");
        }
    }

    // LOOP
    let mut i: i32 = 0;
    loop {
        i += 1;
        if i % 2 == 0 {
            continue;
        } 
        println!("i {i}");
        if i >= 10 {
            break;
        }
    }

    // loop como expressão
    let result: i32 = loop {
        i += 100;
        if i >= 100 {
            break i * 2;
        }
    };
    println!("\nresult: {result}");

    // labels em loops
    let mut contagem: i32 = 0;
    // 
    'meu_externo: loop {
        println!("Contagem = {contagem}");
        let mut faltam: i32 = 100;

        loop {
            println!("Faltam = {faltam}");
            if faltam == 97 {
                break; // break do loop interno
            }
            println!("Contagem = {contagem}");
            if contagem == 2 {
                break 'meu_externo; // break do loop denominado (o loop externo)
            }
            faltam -= 1
        }
        println!("incrementa contagem");
        contagem += 1;
    }
}