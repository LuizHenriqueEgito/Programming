use std::fmt::{Debug, Display};

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

#[derive(Debug, PartialEq, PartialOrd)]
enum MeuEnum {
    VAR1,
    VAR2,
}

// Essa função pode retornar algo que implementa certo trait
// Qualquer struct que implemente Summary poderia ser colocada aqui
fn returns_summarize() -> impl Summary + Debug {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    }
}

// Implementação de métodos condicionada a implementação de traits
#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

// Qualquer <T> serve
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Precisa do Display e do PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Precisa do Display e do PartialOrd
impl<T: Debug + PartialOrd> Pair<T> {
    fn cmp_debug(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}


fn main() {
    // x é algo que implementa Summary
    let x = returns_summarize();
    println!("x é: {:?}", x);
    println!("x summarize: {}", x.summarize());

    let p1: Pair<i32> = Pair::new(11, 12);
    p1.cmp_display();

    let p2: Pair<f64> = Pair::new(11.11, 22.22);
    p2.cmp_display();

    let p3: Pair<MeuEnum> = Pair::new(MeuEnum::VAR1, MeuEnum::VAR2);
    p3.cmp_debug();
}