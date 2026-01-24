/*
O Option está definido na biblioteca padrão
<T> é um tipo generico

// Ou algo ou none
enum Option<T> {
    None,
    Some(T),
}

O match abre a caixa. Se ele encontrar algo, 
ele "batiza" esse conteúdo com o nome que você escolheu 
(neste caso, i para o conteúdo de x e j para o conteúdo de y).

i e j nascem dentro do match.
O que isso significa em português?

“Se x for Some(i)
e y for Some(j)
então some i + j”

i e j são variáveis criadas pelo padrão do match.
Abre a caixa → pega o valor → dá um nome para ele
*/
fn somar(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    match (x, y) {
        (Some(i), Some(j)) => Some(i+j),
        (Some(i), None) => None,
        (None, Some(j)) => None,
        (None, None) => None,
    }
}
// Existe uma forma ainda melhor de fazer isso
fn somar_v1(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    Some(x? + y?)
}

fn main() {
    let number = Some(5);
    let not_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y -> isso quebra pois Some(5) não é um int verdadeiramente é uma estrutura (enum)
    println!("Resultado função somar: {:?}", somar(number, number));
    println!("Resultado função somar: {:?}", somar(number, None));
    println!("Resultado função somar: {:?}", somar(None, number));
    println!("Resultado função somar: {:?}", somar(None, None));

    println!("Resultado função somar: {:?}", somar_v1(number, number));
    println!("Resultado função somar: {:?}", somar_v1(number, None));
    println!("Resultado função somar: {:?}", somar_v1(None, number));
    println!("Resultado função somar: {:?}", somar_v1(None, None));
}