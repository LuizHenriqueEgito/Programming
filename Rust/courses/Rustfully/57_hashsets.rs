use std::collections::HashSet;  // são como sets no python (possui diference, union, symetric diference etc...)

fn main() {
    let numbers: HashSet<i32> = HashSet::new();
    numbers.insert(10);
    numbers.insert(20);
    numbers.insert(10);  // não contém duplicatas
    println!("numbers={:?}", numbers);

    let n = HashSet::from([10, 20, 10]);
    println!("n={:?}", n);
}
