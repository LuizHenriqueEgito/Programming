/*
Signed   Unsigned (só positivos)
i8       u8
i16      u16
i32      u32
i64      u64
i124     u124
isize    usize -> usa o default do sistema (32 bits ou 64)
Para float temos (f32 iou f64)
*/
const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);


fn main() {
    let chassi: i32 = 123456;
    let acel_max: f64 = 3.0;
    let acel_min: f64 = -10.0;
    let vel_max: f32 = VELOCIDADE_MAXIMA as f32; // isso pode
    let comprimento: i32 = 4;
    let posicao_atual: f64 = -100.0;
    let vel_atual: f64 = 0.0;
    let acel_atual: f64 = 0.0;

    // add
    let sum: f64 = posicao_atual + 10.0;

    // sub
    let difference: f64 = vel_atual - 4.3;

    // mul
    let product: i32 = comprimento * 2;

    // div
    let quotient: f64 = acel_atual / 2.0;
    let floored: i32 = 2 / 3;  // é truncado

    // resto da divisão
    let remainder: i32 = 43 % 5;

    // transformando tipos
    let x: f64 = 132.5;

    let y = x + 88f64;  // isso funciona
    let y = x + 88 as f64;  // isso funciona

    // trunc: trunca
    // round: arredonda
    // ceil: arredonda para cima
    // floor: arredonda para baixo
    println!(
        "trunc {}, round {}, ceil {}, floor {}", 
        x.trunc(), x.round(), x.ceil(), x.floor()
    );

    // outros tipos
    let t: bool = true;
    let f: bool = false;

    let x: bool = t && f;
    let y: bool = t || !f;
    let z: bool = 12 > 13;
    
    let c: char = 'z';
    let _c: char = 'z';
    println!("bool: {x}, char {c}");
}