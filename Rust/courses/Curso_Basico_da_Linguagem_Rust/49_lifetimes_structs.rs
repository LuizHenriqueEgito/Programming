use std::fmt::Display;


#[derive(Debug)]
struct Pessoa<'a> {
    name: &'a str,
    cpf: &'a str
}

impl<'a> Pessoa<'a> {
    fn get_cpf(&self, x: &str) -> &str {
        self.cpf
    }
    
    fn longest(&self, x: &'a str) -> &str {
        if x.len() > self.name.len() {
            x
        } else {
            self.name
        }
    }
}

// 'static significa que a referência é válida durante toda a execução do programa
// Por exemplo, todo string literal é 'static
fn add() -> &'static str {
    "added"
}

// Exemplo com Generics para o tipo e tempo de vida
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let meu_nome: String = String::from("Nuna");
    let p1: Pessoa = Pessoa {
        name: &meu_nome,
        cpf: "123456789"
    };
    println!("pessoa: {:?}", p1);

    let p2: Pessoa;
    {
        let cpf_estatico = "789456123";
        let cpf_blobo = String::from("112233445566778899");
        p2 = Pessoa {
            name: meu_nome.as_str(),  // Tem o tempo de vida função main()
            // cpf: &cpf_blobo  Tem o tempo de vida deste bloco
            cpf: cpf_estatico,  // Tem o tempo de vida estático (não precisa de &cpf_estatico)
        }
    }
    println!("pessoa 2 : {:?}", p2);
    println!("Longest: {}", p1.longest("abc"));
    println!("Added: {}", add());
}