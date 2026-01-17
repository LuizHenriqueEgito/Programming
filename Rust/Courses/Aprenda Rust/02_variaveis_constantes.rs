// Constantes
/*
Constantes no Rust na hora de compular ele basicamente hardcoda sua constante no binario alterando
o valor isso dá muito mais velocidade mas se sua constante for grande/pesada, o binario pode ficar
maior.
*/
const SECONDS_IN_MINUTE: u32 = 60
const MINUTES_IN HOUR: u32 = 60
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * SECONDS_IN_HOUR

// RAII
fn main() {
    let total_00: i32 = 30;
    println!("Trabalhou: {} horas", total_00);

    let mut total_01: i32 = 44;
    println!("Trabalhou: {} horas", total_01);
    total_01 = 15;
    println!("Trabalhou: {} horas", total_01);

    let total_em_segundos = total_01 * SECONDS_IN_HOUR;
    println!("Trabalhou {} segundos!", total_em_segundos);

    // se quiser raproveitar o nome da variavl basta dar um let
    let total_01: &str = "Quarenta"
    println!("Trabalhou: {} horas", total_01);
    {
        // escopo interno
        let total_00: i32 = 15
        println!("[ESCOPO INTERNO] Trabalhou: {} horas", total_00);
    }
}  // fim
// Drop: aqui total é destruido