fn set_brightness(brightness: Option<i32>) {
    match brightness {
        Some(value) => println!("The brightness was set to {value}"),
        _ => ()
    }
}

// podemos fazer a mesma coisa só usando if let:
fn set_brightness_iflet(brightness: Option<i32>) {
    if let Some(value) = brightness {
        println!("The brightness was set to {value}");
    } else {  // podemos usar o else ainda
        println!("No brightness set.")
    }
}

fn main() {
    let user_input: Option<i32> = Some(10);
    set_brightness(user_input);
    set_brightness_iflet(user_input);
}