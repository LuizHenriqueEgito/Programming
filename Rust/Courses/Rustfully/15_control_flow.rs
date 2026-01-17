fn check_length(password: &str) {
    let lenght = password.len();

    if lenght >= 10 {
        println!("'{password}' is long enough!")
    } else {
        println!("'{password}' is NOT long enough! Please add more characteres...")
    }
}

fn long_enough(password: &str) -> bool {
    let lenght = password.len();

    if lenght >= 10 {
        true
    } else {
        false
    }
}
fn main() {
    check_length("Bobhashat123");
    check_length("Bob123");

    if long_enough("Bob123") {
        println!("'{password}' is long enough!")
    } else {
        println!("'{password}' is NOT long enough! Please add more characteres...")
    }

    if long_enough("Bobhashat123") {
        println!("'{password}' is long enough!")
    } else {
        println!("'{password}' is NOT long enough! Please add more characteres...")
    }
}