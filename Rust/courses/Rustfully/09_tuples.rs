// tuplas tem tamanho fixo
fn main() {
    let data: (i8, f32, bool) = (10, 3.5, false);
    println!("Data={:?}", data);  // print em modo de debug

    // acessando os dados da tupla
    let (n, d, b) = data;  // desempacota igual ao python
    
    let first = data.0;  // first == n
    let second = data.1;  // second == d
    let last = data.2;  // last == b

    let coordinates: (f32, f32) = (2.5, 1.5);
    println!("The treasure is located here: {:?}", coordinates);

    let empty: () = ();  // tupla vaiza
}
