/*
Vector
String
Hash Map
*/

fn main() {
    // VECTORS
    // reserva na HEAP 5 slots
    let lista: [u8; 5] = [1, 2, 3, 4, 5];
    println!("Valor na posição 2: {}", lista[2]);

    let mut numeros: Vec<u8> = Vec::new();
    numeros.push(1);
    numeros.push(2);
    numeros.push(3);
    numeros.push(4);
    numeros.push(5);
    println!("Valores do vetor: {:?}", numeros);
    for n in numeros {
        println!("{}", n);
    }
    println!('------------------------');

    // STRING
    let texto: String: String::from("Egito");
    texto.push_str(" é meu nome.");
    println!("{}", texto);
    println!('------------------------');
    
    // HASHMAP
    let mut mapa: HashMap<String, &str> = HashMap::new();
    mapa.insert(String::from("nome"), "Egito");
    mapa.insert(String::from("url"), "egito.com.br");
    println!("{:?}", mapa);

    match mapa.get(&"url".to_string()) {
        Some(valor) => {
            println!("{}", valor);
        },
        None => {
            println!("Não foi possivel encontrar o valor.")
        }
    }
}