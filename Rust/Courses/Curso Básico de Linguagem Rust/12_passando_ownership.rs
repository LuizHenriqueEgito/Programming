fn main() {
    let s: String = String::from("hello");

    recebe_ownership(s.clone());
    println!("{}", s);  // isso funciona

    recebe_ownership(s);  // Propriedade do String entregue para a função
    // println!("{}", s);  // s não é mais válido

    let x: i32 = 5;
    recebe_copia(x);  // x é apenas copiado para a função
    println!("{}", x);  // x continua valido
}  // s não é dono do String, então ele não é liberado da memória neste ponto

fn recebe_ownership(a_string: String) { // aqui é como se a_string = s e ai s passa seu valor movemos de s para a_string
    println!("{}", a_string)
}

fn recebe_copia(a_inter: i32) {
    println!("{}", a_inter);
}