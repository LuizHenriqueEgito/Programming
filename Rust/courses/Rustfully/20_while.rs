fn main() {
    let mut number = 5;

    while number > 0 {
        number -= 1;
        println!("Number={:?}", number);
    }

    println!("Loop finished!");

    let mut n = 10;
    while n > 0 {
        n -= 1;

        if n == 5 {
            println!("Skipping 5!");
            continue;
        }
        println!("n={:?}", n)
    }

}