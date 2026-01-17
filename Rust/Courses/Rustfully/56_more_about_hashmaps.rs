use std::collections::HashMap;

fn main() {
    let mut items: HashMap<String, i32> = HashMap::new();
    items.insert(String::from("Bob"), 10);
    println!("items={:?}", item);
    items.entry(String::from("Bob")).or_insert(20);  // caso não exista inclua
    items.entry(String::from("James")).or_insert(13);
    println!("items={:?}", item);

    let text = "Bob says: Bob said that bob said that bob didn't say anything.";
    let mut map: HashMap<String, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word.to_lowercase()).or_insert(0);
        *count += 1
    }
    println!("map={:?}", map);
}