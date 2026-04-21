const MINHA_CONSTANTE: i32 = 1 * 60 * 60;

fn main() {
    println!("Inicio do programa");
    println!("Eu tenho uma constante: {MINHA_CONSTANTE}");
    let x: i32 = 5;
    println!("O valor de x é: {x}");
    // x = 6; // isso pode? Não pode pois no let não veio seguido de mut
    let x: i32 = 42; // isso pode pois com let vc está destruindo o primeiro x e recriando
    println!("O valor de x agora é: {x}");
    let mut y: i32 = 5;
    println!("O valor de y é: {y}");
    y = 6; // isso pode pois criamos y usando mut
    println!("O valor de y agora é: {y}")
}