/*
--------------------
--- Stack e Heap ---
--------------------
STACK:
- Precisa ser pequena (Tamanho da memória)
- Rápida
- Tamanho fixo
- Organizada LIFO (Last In First Out)
- Compilador enxerga a Stack, não enxerga a Heap
- Toda variavel que criamos, ou criamos ela direto na Stack ou criamos um ponteiro
na Stack que aponta para o valor dela guardado na Heap.
x = 10  // fica na Stack, é um tipo por valor
y = "Olá"  // y é um tipo por referencia y -> endereço na Heap -> endereço na Heap -> "Olá"
Dessa forma Stack serve para dados simples de tamanho fixo.

HEAP:
- Memória grande
- Tamanho dinâmico
- Mais lenta do que a Stack (por ser maior)
- Desorganizada
- Usada quando o tamanho do objeto não é conhecido, você não pode jogar algo desconhecido
na Stack pois como ela é pequena é melhor não correr o risco de encher ela.
Dessa forma Heap serve para dados grandes ou que podem crescer.

---
No Rust as coisas vão para a Stack por padrão, você usa o Heap apenas com tipos especificos.
O tamanho é fixo -> vai para a Stack, se não vai para o Heap

Tipo que ficam na Stack:
(Primitivos)
i32, i64, u32, f64, bool, char

(Tuplas - Se todos os items forem fixos)

(Referencias)
&T -> estão na Stack

Tipos que vão para o Heap:
- (String)
- (Vec<T>)
- Box<T>
Para todos os valores que estão no Heap existe a sua referencia na Stack para acessa-los
*/

fn main() {
    let x: i32 = 10;  // Vai para a Stack
    let t: (i32, f64, bool) = (1, 2.0, true);  // vai para a Stack

    /*
    Você deve estar se perguntando porque utilizar &str em uma string que já está na Stack.
    "oi" já está salvo na memoria do programa ao ser compilada, já tem um tamanho pré definido
    let x: &str = "oi";
    │   │  │      │
    │   │  │      └─ String literal (dados read-only)
    │   │  └──────── Tipo: referência para str
    │   └─────────── Nome da variável
    └─────────────── Declaração de variável

    ┌─────────────────────────────────────────┐
    │    SEGMENTO DE DADOS (Read-Only)        │  ← Parte do binário
    │    (Compiled into the binary)           │
    ├─────────────────────────────────────────┤
    │  "oi\0"                                 │  ← String literal
    │  endereço: 0x00401000                   │
    └─────────────────────────────────────────┘

    ┌─────────────────────────────────────────┐
    │            STACK                        │
    ├─────────────────────────────────────────┤
    │  x: &str                                │
    │  ┌────────────┬─────────┐               │
    │  │ ptr        │ len     │               │
    │  │ 0x00401000 │ 2       │               │
    │  └────────────┴─────────┘               │
    │       │                                 │
    │       └──────┐                          │
    └──────────────┼──────────────────────────┘
                   │
                   ▼
                  "oi" (no segmento de dados)
        Explicação:

        1. String literal "ola claude":
            - Está compilada no binário do programa
            - Localização: Segmento de dados read-only (.rodata section)
            - Existe durante toda a execução do programa ('static lifetime)
            - NÃO está na stack nem heap!


        2. Variável x:
            - Está na stack
            - Contém 2 valores:

        Ponteiro para o início da string (endereço 0x00401000)
        Tamanho da string (2 bytes)
    str sem & não pode ser usado diretamente!
    */
    let string_stack: &str = "oi";  // String imutavel

    // o ponteiro `string` está na Stack o valor "Hello" está no Heap
    /*
    STACK:
    string → endereço 0x123

    HEAP:
    0x123 → "Hello"
    */
    let string: String = String::from("Hello"); 
    println!("x: {}", x);
    println!("t: {:?}", t);
    println!("string: {}", string);
    println!("string_stack: {}", string_stack);
}