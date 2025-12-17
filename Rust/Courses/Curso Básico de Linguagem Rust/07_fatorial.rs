fn fatorial_classico(n: i64) -> i64 {
    let mut fatorial: i64 = 1;
    for i in 2..=n {
        fatorial *= i;
    }
    fatorial
}

fn fatorial_recursivo(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }
    n * fatorial_recursivo(n-1)
}

fn fatorial_iterador(n: i64) -> i64 {
    (1..=n).product()
}

fn main() {
    let x: i64 = 5;
    println!("Fatorial classico: {}! é {}", x, fatorial_classico(x));
    println!("Fatorial recursivo: {}! é {}", x, fatorial_recursivo(x));
    println!("Fatorial iterador: {}! é {}", x, fatorial_iterador(x));
}