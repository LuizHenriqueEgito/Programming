fn main() {
    // Imutable variable
    let x: i32 = 5;
    println!("The value of x is: {}", x);
    // não podemos fazer x = 6 pois x é imutavel
    let mut y: i32 = 10;
    y = 25;  // podemos fazer isso pois mut deixa a variavel mutavel
    println!("The value of y is: {}", y);

    // contant
    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.14159;
    println!(
        "Max points: {}, pí: {}", MAX_POINTS, PI
    );

    // Shadowing
    let z: i32 = 20;
    println!("The value of z is: {}", z);
    let z: i32 = z + 5;  // shadowing the previous value
    println!("The value of z is: {}", z);

    // Inner scope shadowing
    {
        let z: i32 = 25;  // shadowing in an inner scope
        println!("The value of z is: {}", z);
    }

    let z: &'static str = "Now I'm a string";
    println!("The value of z is: {}", z);

    let spaces: &'static str = "    ";
    let spaces: usize = spaces.len();
    println!("The length of spaces is: {}", spaces);
}