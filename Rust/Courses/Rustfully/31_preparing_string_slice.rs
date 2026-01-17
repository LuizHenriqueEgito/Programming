fn get_firts_word(sentece: &String) -> usize {
    let bytes = setence.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    sentece.len()
}

fn main() {
    let mut sentence = String::from("Bob doesn't care.");
    let first_word = get_firts_word(&sentece);
    println!("first_word={:?}", first_word);
    sentece.clear();
}