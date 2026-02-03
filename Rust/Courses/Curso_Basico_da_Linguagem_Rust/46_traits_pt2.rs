use std::fmt::Debug;

trait Summary {
    fn summarize(&self) -> String {
        String::from("(comportamento default...)")
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// A função aceita qualquer tipo de dado que
// implementa o trait Summary
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// outra forma de fazer a função acima
// aceito qualquer T desde que tenha a trait Summary
fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Se eu quiser que ele implemente mais de um trait
fn notify_duplo(item: &(impl Summary + Debug)) {
    println!("notify_duplo: {} {:?}", item.summarize(), item);
}

// Mesma coisa que a função acima
fn notify_duplo_2<T: Summary + Debug>(item: &T) {
    println!("notify_duplo_2: {} {:?}", item.summarize(), item);
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
    notify_2(&tweet);
    notify_duplo(&tweet);
    notify_duplo_2(&tweet);
}