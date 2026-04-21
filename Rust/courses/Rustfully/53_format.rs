fn main() {
    let mut text: String = String::from("Bob");
    let ending: String = String::from(" was here!");
    text.push_str(&ending);
    println!(text);

    let s1: String = String::from("Hello, ");
    let s2: String = String::from("Bob!");
    let s3: String = s + &s2;
    println!(s3)

    let s4: String = s1 + "-" + &s2 + "-" + &s3;
    println!(s3);

    let s5: String = format!("{s1}-{s2}-{s3}");
    println!(s5);
}