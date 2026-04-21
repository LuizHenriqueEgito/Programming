// Tudo aqui deve ser tipado
#[derive(Debug)]  // é preciso disso para printar
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user_v0(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// quando o nome do parametro for exatamente igual ao nome da Struct
// podemos usar assim, mais abreviado, fica sub-entendido
// ele vai transformar &str em String usando o .to_string()
fn build_user_v1(email: &str, username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1: User = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        sign_in_count: 1,
    };
    println!("User 1: {:?}", user1);
    // acessando um "atributo" da minha struct
    println!("Email: {}", user1.email);
    // como ele tem mut eu posso alterar algum atributo
    user1.email = String::from("user1@outlook.com");

    let user2: User = build_user_v0(String::from("user2@yahoo.com.br"), String::from("user2"));
    println!("User 2: {:?}", user2);
    let user3: User = build_user_v1("user3@email.com", "user3");
    println!("User 3: {:?}", user3);
}