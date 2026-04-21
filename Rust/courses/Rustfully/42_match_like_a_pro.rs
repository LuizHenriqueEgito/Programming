// None ou Some(T) vem de Option<T>
// use _ => quando não for nenhuma das opções que você espera.
fn user_exists(user: Option<&str>) -> bool {
    match user {
        None => {
            println!("Please insert a username to search for.")
            false
        }
        Some(user) => {
            println!("Looking for user...");
            println!("user: {user} found!")
            true
        }
    }
}

fn main() {
    let result = user_exists(None);
    println!("result={:?}", result);
}