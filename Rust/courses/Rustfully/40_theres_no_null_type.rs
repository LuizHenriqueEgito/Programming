// No rust não possuimos null, none, etc
// Um valor ou existe, ou o código não compila.
// Para representar o "Pode não existir" usamos Option<T>
// O que é o Option<T>, é um enum da stdlib:
enum Option<T> {
    Some(T),  // existe
    None  // não existe
}

// Se algo pode falhar, faltar, não existir ou ser inválido, use Option<T>

fn main() {
    let x: Option<i32> = Some(10);
    let y: Option<i32> = None;
}