// isso não execut nada
trait Speak {
    fn falar(&self);
}

// quem cumpre esse "protocolo"
struct Person;

impl Speak for Person {
    fn falar(&self) {
        println!("Hello!")
    }
}
// Função que aceita qualquer tipo, desde que implemente o trait
fn talking<T: Speak>(x: T) { // aceito qualquer T mas ele precisa implementar o Speak
    x.falar();
}

// mesma coisa do de cima mas sem o açucar sintatico
fn talking_(x: impl Speak) {
    x.falar();
}

fn main() {
    let p = Person;
    talking(p); // funciona
    // talking(5) não compila...
}