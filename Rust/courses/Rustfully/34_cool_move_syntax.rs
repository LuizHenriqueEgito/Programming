struct User {
    id: i32,
    username: String,
    email: String,
}

fn main() {
    let user = User {
        id: 0,
        username: String::from("bob_123");
        email: String::from("bob@123.gmail")
    };

    let update_user = User {
        id: 1,
        ..user  // isso pega o resto de user que não queremos alterar
    };

    println!("user={:?}", update_user);
}