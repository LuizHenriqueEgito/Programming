fn numero_primo_while(num: u32) -> bool {
    if num <= 1 {
        return false
    }
    let limite: u32 = (num as f64).sqrt() as u32;
    let mut d: u32 = 2;
    while d <= limite {
        if num % d == 0 {
            return false
        }
        d += 1;
    }
    true
}

fn numero_primo_for(num: u32) -> bool {
    if num <= 1 {
        return false
    }
    let limite: u32 = (num as f64).sqrt() as u32;
    for d in 2..=limite {
        if num % d == 0 {
            return false
        }
    }
    true
}

fn numero_primo_for_v2(num: u32) -> bool {
    if num <= 1 {
        return false
    } else if num == 2 {
        return true
    } else if num == 3 {
        return true
    } else if num % 2 == 0 {
        return false
    } else if num % 3 == 0 {
        return false
    }
    let limite = (num as f64).sqrt() as u32;
    for d in (5..=limite).step_by(2) {
        if num % d == 0 {
            return false
        }
    }
    true
}

fn main() {
    println!("numero {} é primo {}", 13, numero_primo_while(13));
    println!("numero {} é primo {}", 13, numero_primo_for(13));
    println!("numero {} é primo {}", 13, numero_primo_for_v2(13));
}