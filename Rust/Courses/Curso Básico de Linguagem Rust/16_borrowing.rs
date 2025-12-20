fn main() {
    let palavra: String = String::from("abacaxi");
    // Move: passa a propriedade do 'String'
    let len1: usize = calcula_tamanho_move(palavra.clone());  // só assim para funcionar
    println!("O tamanho de '{}' é '{}'", palavra, len1);

    let palavra: String = String::from("tomate");
    // Referencia: não passa a propriedade do 'String'
    let len2: usize = calcula_tamanho_referencia(&palavra);
    println!("O tamanho de '{}' é '{}'", palavra, len2);

    // É a mesma coisa que na linguagem C?
    let x: i32 = 11;
    soma_900(&x);
    soma_900(&22);
}

// A mpropriedade do 'String' é recebida pela função
// Essa função recebe e destroi o String
fn calcula_tamanho_move(s: String) -> usize {
    s.len()  // -> isso calcula o número de bytes do String
    // .char().count() retorna o número de caracteres do String
    // s fica invalido
    // s tinha a propriedade do 'String', drop do 'String'
}

// Um emprestimo do 'String' é recebido pela função, e não a propriedade do 'String'
fn calcula_tamanho_referencia(s: &String) -> usize {
    s.len()
    // s fica invalido
    // s não tinha a propriedade do 'String', nenhum drop acontece
}

// referencia (emprestimo) não é a mesma coisa que endereço '&' em C
// ATENÇÃO: i & aqui é um EMPRESTIMO do valor
fn soma_900(ref_int: &i32) {
    let c_a: i32 = *ref_int + 900;
    let s_a: i32 = ref_int + 900;
    println!("Com asterisco {c_a} | Sem asterisco: {s_a}");
}