// ownership está relacionado ao escopo da variavel
// toda variável tem um escopo
// a variável é valida somente dentro daquele escopo
// escopo é definido pelo momento de definição e destruição

fn exemplo() {
    let s1: &str = "primeira string literal";
    {
        let s2: &str = "segunda string literal";
        println!("O valor de s1 é {s1}");
        println!("O valor de s2 é {s2}");
    }
    println!("O valor de s1 é {s1}");
    // println!("O valor de s2 é {s2}")  isso dá erro pois s2 não existe neste escopo
}

fn main() {
    exemplo();
}