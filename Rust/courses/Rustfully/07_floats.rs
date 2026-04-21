fn main() {
    let pi: f32 = 3.1415927;
    let decimal: f64 = 2.718281828459045;

    println!("PI: {pi}");
    println!("Decimal: {decimal}");

    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let sum: f64 = a + b;
    println!("Sum: {sum}");
    println!("{}", sum == 0.3)  // isso da false
}