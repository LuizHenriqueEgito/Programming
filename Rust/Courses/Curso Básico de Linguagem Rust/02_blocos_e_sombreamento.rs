fn main() {
    // BLOCOS
    println!("Inicio do programa");
    const X: i32 = 5;
    let y: i32 = 6;
    let mut z: i32 = 7;
    z += 1; // z = z + 1
    println!("No início os valores são: X={X}, y={y}, z={z}");
    { // bloco interno (é um escopo interno)
        const X: i32 = 55;
        let y: i32 = 66;
        let mut z: i32 = 77;
        z = z + 1;
        println!("Dentro do bloco interno os valores são: X={X}, y={y}, z={z}");
    }
    println!("Depois do bloco interno os valores são: X={X}, y={y}, z={z}");

    // SOMBREAMENTO
    let x: i32 = 5;
    println!("O valor de x é {x}");
    let x: i32 = x + 1; // isso pode acontecer
    println!("O valor de x agora é {}", x);
    let spaces: &str = "     ";
    let spaces: usize = spaces.len(); //usize: inteiro sem sinal
    println!("O valor de spaces é: {spaces}");

    let mut spaces2: &str = "     ";
    println!("O valor de spaces2 é: {spaces2}");
    spaces2 = "qwerty";
    println!("O valor de spaces2 é: {}", spaces2);
}