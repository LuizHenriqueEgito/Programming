enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin_penny = Coin::Penny;
    let coin_penny = value_in_cents(coin_penny);
    println!("Coin: {:?}", coin_penny);

    let coin_nickel = Coin::Nickel;
    let coin_nickel = value_in_cents(coin_nickel);
    println!("Coin: {:?}", coin_nickel);

    let coin_dime = Coin::Dime;
    let coin_dime = value_in_cents(coin_dime);
    println!("Coin: {:?}", coin_dime);

    let coin_quarter = Coin::Quarter;
    let coin_quarter = value_in_cents(coin_quarter);
    println!("Coin: {:?}", coin_quarter);
}