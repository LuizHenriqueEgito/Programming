/*
O Match no Rust requer que a cubramos todas as possibilidades
use `other` para representar outros casos e ainda ter o valor
use `_` para representar outros valores mas sem acessar o valor.
*/


fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => println!("Add Fancy Hat"),
        7 => println!("Remove Fancy Hat"),
        other => println!("Player moved: {other}"),  // todos os outros casos
    }

    match dice_roll {
        3 => println!("Add Fancy Hat"),
        7 => println!("Remove Fancy Hat"),
        _ => println!("Reroll"),  // todos os outros casos
    }

    let nothing: () = match dice_roll {
        3 => println!("Add Fancy Hat"),
        7 => println!("Remove Fancy Hat"),
        _ => (),  // unit value (entre aspas é o "nada" no Rust)
    };
    println!("Nothing: {:?}", nothing);
}