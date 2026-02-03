fn devolve_ownership() -> String {
    // Conteúdo de 'algo' é retornado para a função chamadora, semântica 'move' pois é um 'String'
    let algo: String = String::from("AAA");  // Variável 'algo' fica invalida
    algo
}

fn recebe_e_devolve_ownership(a_string: String) -> String {
    println!("{}", a_string);
    // Contéudo de 'a_string' é retornado para a função chamadora, semântica 'move' pois é um 'String'
    // Variável 'a_string' fica inválida
    a_string
}


fn main() {
    let s1: String = devolve_ownership();  // valor de retorno é movido para s1
    let s2: String = String::from("hello");
    // s2 é movido para a função, s2 fica invalido, o retorno da função vai para s3
    let s3: String = recebe_e_devolve_ownership(s2);
    // s1 fica inválido, sua propriedade sofre um drop
    // s2 fica inválido, mas não é dono de nada
    // s3 fica inválido, sua propriedade sofre um drop
    println!("s1: {} | s2 não existe mais| s3: {}", s1, s3);
}
