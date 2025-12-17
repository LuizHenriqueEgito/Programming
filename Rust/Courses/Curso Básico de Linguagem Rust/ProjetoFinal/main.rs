use std::thread::sleep;
use std::time::Duration;

const _VIAH_MARGEM: f64 = 15.0;
const _VIAV_MARGEM: f64 = 15.0;

const VIAH_LARGURA: f64 = 4.0;
const VIAV_LARGURA: f64 = 4.0;

const VIAH_PERIMETRO: f64 = 150.0;
const VIAV_PERIMETRO: f64 = 150.0;

const CARRO_LARGURA: f64 = 2.0;
const CARRO_COMPRIMENTO: f64 = 4.0;

const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);
const ACELERACAO_MAXIMA: f64 = 3.0;
const ACELERACAO_MINIMA: f64 = -10.0;


fn simula_carros(via_carro1: char, acel_carro1: f64, via_carro2: char, acel_carro2: f64) -> bool {
    let chassi1: i32 = 1111;
    let via1: char = via_carro1;
    let _acel_max1: f64 = ACELERACAO_MAXIMA;
    let _acel_min1: f64 = ACELERACAO_MINIMA;
    let vel_max1: f64 = VELOCIDADE_MAXIMA;
    let comprimento1: f64 = CARRO_COMPRIMENTO;
    let mut pos_atual1: f64 = -80.0;
    let mut vel_atual1: f64 = 0.0;
    let acel_atual1:f64;

    let chassi2: i32 = 2222;
    let via2: char = via_carro2;
    let _acel_max2: f64 = ACELERACAO_MAXIMA;
    let _acel_min2: f64 = ACELERACAO_MINIMA;
    let vel_max2: f64 = VELOCIDADE_MAXIMA;
    let comprimento2: f64 = CARRO_COMPRIMENTO;
    let mut pos_atual2: f64 = -80.0;
    let mut vel_atual2: f64 = 0.0;
    let acel_atual2: f64;

    acel_atual1 = acel_carro1;
    acel_atual2 = acel_carro2;

    println!("Inicio da simulação");
    let mut tickms: f64;

    loop {
        sleep(Duration::from_millis(100));  // dorme 100ms
        tickms = 100.0;  // executa 100ms

        // carro 1
        let old_position: f64 = pos_atual1;

        pos_atual1 = pos_atual1 + vel_atual1 * (tickms / 1000.0) + acel_atual1 * (tickms / 1000.0) * (tickms / 1000.0) / 2.0;
        vel_atual1 = vel_atual1 + acel_atual1 * (tickms / 1000.0);

        if pos_atual1 < old_position {  // não anda para tras
            pos_atual1 = old_position;
        }
        if vel_atual1 < 0.0 {  // não anda para tras
            vel_atual1 = 0.0;
        }
        if vel_atual1 > vel_max1 {  // não acelera mais que a vel max
            vel_atual1 = vel_max1;
        }
        println!("Carro1 {} na posição {}{}, velocidade {}, aceleração {}",
            chassi1, via1, pos_atual1, vel_atual1, acel_atual1);
        
        // carro 2
        let old_position: f64 = pos_atual2;

        pos_atual2 = pos_atual2 + vel_atual2 * (tickms / 1000.0) + acel_atual2 * (tickms / 1000.0) * (tickms / 1000.0) / 2.0;
        vel_atual2 = vel_atual2 + acel_atual2 * (tickms / 1000.0);

        if pos_atual2 < old_position {  // não anda para tras
            pos_atual2 = old_position;
        }
        if vel_atual2 < 0.0 {  // não anda para tras
            vel_atual2 = 0.0;
        }
        if vel_atual2 > vel_max2 {  // não acelera mais que a vel max
            vel_atual2 = vel_max2;
        }
        println!("Carro1 {} na posição {}{}, velocidade {}, aceleração {}",
            chassi2, via2, pos_atual2, vel_atual2, acel_atual2);
        
        if via1 == 'H' && via2 == 'H' {
            if colisao_longitudinal(pos_atual1, comprimento1, pos_atual2) {
                println!("Colisão na via H");
                return true
            }
        }

        if via1 == 'V' && via2 == 'V' {
            if colisao_longitudinal(pos_atual1, comprimento1, pos_atual2) {
                println!("Colisão na via V");
                return true
        }

        // detecção colisão no cruzamento
        if via1 != via2 {
            if dentro_cruzamento(pos_atual1, comprimento1, via1) && dentro_cruzamento(pos_atual2, comprimento2, via2) {
                println!("Colisão dentro do cruzmento");
                return true
            }
        }

        if pos_atual1 > comprimento1 + if via1=='H' {VIAV_LARGURA} else {VIAH_LARGURA} {
            break false;
        }

        if pos_atual2 > comprimento2 + if via2=='V' {VIAV_LARGURA} else {VIAH_LARGURA} {
            break false;
        }
    }
    return false;
}
}

fn colisao_longitudinal(posicao_frente: f64, comprimento: f64, posicao_atras: f64) -> bool {
    return posicao_frente - comprimento <= posicao_atras;
}

fn dentro_cruzamento(posicao: f64, comprimento: f64, via: char) -> bool {
    return posicao > 0.0 &&
        posicao <= comprimento + if via=='H' {VIAH_LARGURA} else {VIAV_LARGURA};
}

fn main() {
    println!("inicio do programa");
    simula_carros('H', ACELERACAO_MAXIMA/ 10.0, 'H', ACELERACAO_MAXIMA);
    println!("Fim da simulação");
}