fn modify(&mut text) {
    text.push_str("!");
}

fn main() {
    let mut name = String::from("Bob");
    modify(name);
    println!(name)
}