trait Summary {
    fn summarize(&self) -> String {
        String::from("(comportamento default...)")
    }
}

trait Summary2 {
    fn summarize(&self) -> String {
        String::from("(Summary2 default)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}

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

impl Summary for String {
    fn summarize(&self) -> String {
        format!("Tamanho do String: {}", self.len())
    }
}

struct NaoFazNada {}

impl Summary2 for NaoFazNada {}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };

    println!("Sumário do tweet:\n\t{}", tweet.summarize());

    let s = String::from("qwerty");
    println!("Sumário da string:\n\t{}", s.summarize());

    let nfn = NaoFazNada {};
    println!(
        "Sumário de nfn:\n\t{}",
        Summary2::summarize(&nfn)
    );
}
