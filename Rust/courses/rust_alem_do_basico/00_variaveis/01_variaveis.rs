/*
PROPRIEDADE ÚNICA (OWNERSHIP)
Rust introduz o conceito de propriedade única para gerenciar
a alocação de memória. Cada valor tem exatamente UM "proprietario".
Quando o proprietário sai do escopo, o valor é liberado automaticamente.
*/
fn main() {
    let valor: f64 = 50.0;
    println!("{}", valor);
}