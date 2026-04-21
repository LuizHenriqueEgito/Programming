fn hello(name: &str) {
    println!("Hello, {name}!")
}

fn repeat(text: &str, times: usize) {
    println!("{}", text.repeat(times));
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0  // não precisa de return nem de ;
}

fn add(a: i32, b: i32) -> isize {
    println!("Adding {a} + {b}");
    (a + b) as isize
}

fn main() {
    hello("Nuna");
    hello("Bob");

    repeat("Piramides", 3)

    println!("celsius_to_fahrenheit(30)={}:?", celsius_to_fahrenheit(30));

    let result = add(10, 20);
    dgb!(result);
    println!("result={result}");
}