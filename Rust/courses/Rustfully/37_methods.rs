struct Rectangle {
    width: u32,
    height: u32,
}

// adicionando funcionalidades
// methods sempre exigem self como primeiro parametro
impl Rectangle {
    fn is_valid(&self) -> bool {
        self.widht > 0 && self.height > 0
    }

    fn get_area(&self) -> u32 {
        self.widht * self.height
    }

    fn display(&self) {
        println!("The rectangle is: {} square pixels", self.get_area());
    } else {
        println!("The recttangle is invisible");
    }
}

fn main() {
    let rec = Rectangle {width: 20, height: 30};
    println!("Area: {}", rec.get_area());
}