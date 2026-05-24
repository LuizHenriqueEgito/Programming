fn main() {
    let result = div(100.0, 0.0);
    println!("resultado: {:?}", result);

    match result {
        Some(valor) => {
            println!("O resultado da divisão foi: {}", valor);
        },
        None => {
            println!("Não foi possivel fazer a divisão...")
        }
    }
}

fn div(dividendo: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        return None;
    }
    Some(dividendo / divisor)
}