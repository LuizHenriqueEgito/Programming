fn get_response(input: &str) -> &str {
    let lowered: String = input.to_lowercase();

    if lowered.contains("hello") {
        "Hello there!"
    } else if lowered.contains("how are you") {
        "Good, and you?"
    } else if lowered.contains("good") {
        "Good is good." 
    } else {
        "I don't understand..."
    }
}

fn analyse_number(n: i32) {
    if n > 0 {
        println!("{n} is greater than 0!");
    } else if n > 10 {
        println!("{n} is greater than 10!");
    } else {
        println!("{n} is a cool number!");
    }
}


fn main() {
    get_response("Hello, Bob!");
    get_response("How are you?");
    get_response("Good");
    get_response("Is this a cat in a hat?");

    analyse_number(0);
    analyse_number(11);
    analyse_number(-1);
}