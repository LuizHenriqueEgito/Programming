/*
Em Rust, statements controlam execução; expressions constroem valores.
E o ; é o divisor entre os dois.

Statement
É uma instrução que:
- Executa uma ação
- Não retorna valor
- Termina com ;

Expression
É algo que:
- Avalia para um valor
- pode ser usada no lugar de um valor
- não termina com ;
 */
fn main() {
    // statement
    let x = 10;  // cria uma variável, não produz valor termina com ;
    println!("Hello!");  // outro exemplo, executa e não retorna um valor

    // expression
    let y = {  // if else, blocos {}, e match são todos expressions
        5 + 3  // isso é uma expression
    };  // aqui temos o statement do y
}