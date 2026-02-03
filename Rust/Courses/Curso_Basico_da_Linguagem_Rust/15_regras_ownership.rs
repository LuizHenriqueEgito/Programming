/*
Regras do Ownership
    - Cada valor em rust tem um proprietário (owner)
    - A cada momento pode existir apenas um proprietário para cada valor
    - Quando o proprietário chega ao final do seu escopo o valor é destruido (dropped)


Como a memória de um valor na Heap é liberada?
    - Malloc/Free explícitos (linguagem C)
    - Coletor de lixo / Garbage Colector (Linguagem Java)
No Rust:
    - Conceito de Ownership
    - quando termina o escopo da variável owner/proprietária do valor
*/

fn main() {
    let mut s3: String = String::from("Alo");
    println!("O valor de s3 é {s3}");

    s3.push_str(", mnundo!");
    println!("O vlaor de s3 agora é {}", s3);

    {
        let s4: String = String::from("alo 2");
        println!("O valor de s4 é {s4} e elá só existe nesse bloco");
    }
    // Somente pode haver um dono para cada valor na Heap
    let s5: String = s3;
    println!("Valor de s5 é {s5}");

    // x e y estão na Stack | Conteúdo de x pode ser copiado
    let x: i32 = 10;
    let y: i32 = x;
    println!("x={}, y={}", x, y);
}