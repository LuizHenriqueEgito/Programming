fn main() {
    let connected_to_internet: bool = false;
    let has_cat: bool = true;

    println!("Connected_to_internet={}", connected_to_internet);
    println!("has_cat={}", has_cat);

    let money: i32 = 5_000;
    println!("money > 0 = {}", money > 0);
    if money > 0 {
        println!("You are not broke!")
    }

    let letter: char = 'z';
    let omega: char = 'Ω';
    let heart: char = '❤';

    println!("letter={letter} | omega={omega} | heart={heart}")
}