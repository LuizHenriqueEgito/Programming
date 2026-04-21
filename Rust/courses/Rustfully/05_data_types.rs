fn main() {
    let user_input = "100";
    let converted: u32 = user_input.parse().expect("Could not parse...");

    println!("Converted={converted}")

    // Scalar: um unico valor
    let number: i8 = 10;
    let pi: f32 = 3.1415;
    let turned_on: bool = false;
    let delta: char = 'Δ';

    // Compostos: Agrupam valores
    let coordinates: (f32, f32) = (1.5, 2.5);
    let people: [&str, 3] = ["Bob", "Micheal", "Ashley"];
}