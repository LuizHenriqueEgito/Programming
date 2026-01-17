fn main() {
    let n = 5;
    {
        let n = 10;
        println!("inner n is: {n}");
    }
    println!("outer n is: {n}");

    let spaces = "      ";
    println!("Spaces: {spaces}");
    let spaces = spaces.len();
    println!("Spaces: {spaces}");
}