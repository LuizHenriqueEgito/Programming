/*
fn nome_da_funcoes(parametro1: Tipo1, parametro2: Tipo2) -> TipoDoRetorno {
    // Corpo da função
}

- Não existe valor default em funções do RUST
*/

fn foobar1() -> i32 {
    3
}

fn foobar2() -> i32 {
    return 3
}

// Parametros Opcionais
fn saudacao(nome: &str, saudacao_personalizada: Option<&str>) {
    match saudacao_personalizada {
        Some(s) => println!("{} {}", s, nome),
        None => println!("Olá, {}!", nome),
    }
}

// Retorno de bloco
fn maior_valor(a: i32 , b: i32) -> i32 {
    {
        if a > b {
            a
        } else {
            b
        }
    }
}

// parametros mutaveis
fn incrementa(mut a: i32) -> i32 {
    a += 1;
    a
}

fn main() {
    saudacao("Luiz", Some("Bom dia"));
    saudacao("Egito", None);
    let x = foobar1();
    let y = foobar2();
    println!("{} {}", x, y);
    let w = maior_valor(10, 20);
    println!("O maior valor passado foi: {}", w);
    let c = 1;
    let d = incrementa(c);
    println!("Passado {} devolvido {}", c, d);

}