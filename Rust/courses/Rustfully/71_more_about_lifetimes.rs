struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn main() {
    let movel = String::from("Call me Ishmel. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Excerpt: {}", i.part);
}