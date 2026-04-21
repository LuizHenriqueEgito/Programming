fn main() {
    let names = ["Bob", "Ben", "Betty"];
    for person in names {
        println!("{name} says: hi!");
    }

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let mut power_total = 0;
    for number in numbers {
        let squared = number.pow(2);
        println!("{number}: {:?}", squared);
        power_total += squared;
    }
    println!("power_total: {power_total}")

    // 10 incluso
    for i in 0..=10 {
        println!("{}", i);
    }

    // 10 não incluso
    for i in 0..10 {
        println!("{}", i);
    }

    for i in (0..=10).rev() {
        println!("{}", i);
    }
}