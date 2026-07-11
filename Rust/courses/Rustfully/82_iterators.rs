fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_iter = numbers.iter();

    for number in numbers_iter {
        println!("number = {number}");
    }
    let mapped: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .filter(|&x| x > 5)
        .collect();
    dbg!(mapped);
}