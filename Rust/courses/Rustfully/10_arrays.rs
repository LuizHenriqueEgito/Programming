// arrays deve conter o mesmo tipo de elementos
fn main() {
    let numbers_0: [i8, 5] = [1, 2, 3, 4, 5];
    let numbers_1: [u8, 3] = [1, 2, 3];
    println!("numbers: {:?}", numbers_1);

    let repeat_array = ["Bob"; 100];

    let first = numbers_0[0];
    let second = numbers_0[1];
    let last = numbers_0[4];
}