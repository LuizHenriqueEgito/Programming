use std::io;  // import como no python

// Função que recebe um &str e converte para i32.
// Se a função só precisa LER texto → use &str
// Se precisar ser DONO do texto use String
fn convert_to_int(input: &str) -> i32 {
    input
        .trim()              // Remove espaços e a quebra de linha (\n) adicionada por read_line()
        .parse::<i32>()      // Converte o texto para um número inteiro de 32 bits
        .expect("Erro ao converter para número") // Se não conseguir converter, encerra com esta mensagem
}

// Função que exibe texto, lê a entrada do usuário e retorna como String.
fn read_number(prompt: &str) -> String {
    println!("{}", prompt); // Exibe o texto

    let mut buffer = String::new(); // Cria uma String vazia para armazenar a entrada

    io::stdin()                     // Acessa o stdin (entrada padrão)
        .read_line(&mut buffer)     // Lê a linha digitada e escreve dentro de buffer (por isso &mut)
        .expect("Erro ao ler entrada"); // Se falhar, encerra o programa com esta mensagem

    buffer // Retorna a string lida (ownership movido para quem chamou)
}

// Função principal do programa
fn main() {
    let n1 = read_number("Passe um número:");        // Lê o primeiro número como String
    let n2 = read_number("Passe outro número:");     // Lê o segundo número como String

    let i1 = convert_to_int(&n1);   // Converte n1 para inteiro (passando como &str via &n1)
    let i2 = convert_to_int(&n2);   // Converte n2 para inteiro

    // Compara os dois números convertidos
    if i1 > i2 {
        println!("O número {} é maior que {}", i1, i2); // Caso i1 seja maior
    } else {
        println!("O número {} é menor ou igual que {}", i1, i2); // Caso contrário
    }
}
