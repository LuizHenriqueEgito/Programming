/*
RUST faz a pergunta:
A referência que você retorna vive pelo menos tanto quanto quem está usando ela?
E ele só aceita o código se você provar isso. Ai que entra o `lifetime`.

Na função longest `'a` é o menor entre x e y, isso pois:
=> Imagine dois amigos x e y em uma festa:
    - x fica até as 22h
    - y fica até as 20h
Você diz:
"Vou te emprestar o carro que pertence a um deles (x ou y) para outro amigo"
A regra que você deve seguir é:
- "Você só pode usar o carro até as 20h!"
Mesmo que no fim o carro seja do amigo que fica até 22h. Isso evita pegar o carro de quem
já foi embora.

Toda referência vai ter um tempo de vida igual ou menor ao dado que ela está apontando
na memória.

Notação: 'a, ..., 'z (por padrão começa em a e vai até z)

Resumindo: Em RUST uma referência (x = &y), (x) não pode e não vai existir se
y não existir mais.
*/

/*
Uma função precisa de lifetimes se:
- Recebe referências
- Retorna uma referência
- O retorno depende dos parâmetros
*/

/*
Toda referencia em Rust tem um tempo de vida, o tempo no qual ela é válida
Na maioria dos casos o compilador controla sozinho o tempo de vida dos valores
As vezes são necessárias anotações do programador para ajudar o compilador
*/

/*
Em funções a sintaxe de lifetime busca conectar os tempos de vida dos vários parâmetros
com o tempo de vida do retorno da função.

`s1` e `s2` podem ter tempos de vida diferentes
o compilador não sabe qual deles será retornado
o compilador não sabe qual o tempo de vida de `result`

É por isso que usamos lifetimes eles ajudam o compilador a entender quanto tempo os
emprestimos podem durar.
*/

// Diz para o compilador: O tempo de vida do retoro será o menor tempo
// de vida entre os argumentos
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// outro exemplo que devemos usar lifetimes
// o compilador não olha algoritmo por conta disso precisamos ajudar
// isso aocntece com os emprestimos o compilador não sabe até quando um deles vai viver
// precisamos ajudar ele a entender isso.
fn lifetimes_fn<'a>(s1: &'a str, s2: &str) -> &'a str {
    s1
}

fn main() {
    let y: i32 = 9;
    let mut r: &i32;
    {
        let x: i32 = 5;  // Tempo de vida de `x` limitado a este bloco
        r = &x;  // `x` does not live long enough
    }
    // println!("r: {}", r);  Aqui o `x` não existe mais
    r = &y;
    println!("r: {}", r);  // Aqui `y` ainda existe

    let s1: String = String::from("abcdef");
    let s2: &str = "xyz";
    let result: &str = longest(s1.as_str(), s2);  // s1 e s2 são válidos atpe o final do programa
    println!("The longest string is {}", result);

    // Borrow checker vê `resultado` com tempo de vida apenas no bloco, e isso faz com que seja ok
    let s1 = String::from("longolongolongo");
    {
        let s2 = String::from("curto");
        // Pior caso é quando o resultado for s2, mas é ok pois só vemos ele dentro do bloco
        let result: &str = longest(s1.as_str(), s2.as_str());
        println!("The longest string is {}", result);
    }

    // Borrow checker usa o menor tempo de vida, `resultado` pode ter s2, então da erro pois saimos do bloco
    let s1 = String::from("longlonglong");
    let result: &str = "...";
    {
        let s2 = String::from("curto");
        // Pior caso é quando retorna s2, mas s2 não vive fora do bloco => Então da erro
        // resultado = longest(s1.as_str(), s2.as_str())
    }
    println!("The longest string is {}", result);
}