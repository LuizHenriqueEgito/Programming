#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Mensagem chamada é: {:?}", &self);
    }
}

fn main() {
    let msg = Message::Write(String::from("Alow"));
    msg.call();
}