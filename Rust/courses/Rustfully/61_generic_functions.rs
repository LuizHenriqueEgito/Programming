// <T>: T vem de tipo e isso caracteriza um tipo genérico
fn get_first<T>(list: &T) {
    &list[0]
}

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3];
    let chars: Vec<char> = vec!['a', 'b', 'c'];

    println!(get_first(&numbers));
    println!(get_first(&chars));
}