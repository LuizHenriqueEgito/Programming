fn main() {
    let array: [i32; 5] = [11, 22, 33, 44, 55];
    let slice: &[i32] = &array[1..=3];  // 3º incluso

    println!("{:?}", slice);
}