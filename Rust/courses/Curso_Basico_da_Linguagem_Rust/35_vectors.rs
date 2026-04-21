/*
Aloca uma área de memória no HEAP para salvar elementos
<tipo> é uma maneira de colocar tipos genéricos no rust

A semantica Copy e Move continua valendo aqui com vetores.
*/

fn main() {
    let vec_1: Vec<i32> = Vec::new();  // sempre precisamos definir o tipo 
    println!("{vec_1:?}");

    let mut vec_2: Vec<i32> = Vec::new();
    vec_2.push(0);  // Acrescenta valores no seu vetor
    vec_2.push(1);
    vec_2.push(2);
    println!("{vec_2:?}");

    let vec_3: Vec<i32> = vec![0, 1, 2];  // Macro que crie o vetor
    println!("{vec_3:?}");

    let vec_4: Vec<&str> = vec!["aaa", "bbb", "ccc"];
    println!("{vec_4:?}");

    // Acessando os elementos do meu vector
    println!("{}", vec_4[1]);

    // podemos pegar emprestado
    let emprestimo_v4: &str = &vec_4[0];
    println!("Peguei emprestado: {emprestimo_v4}");
    println!("{}", vec_4[0]);
    // podemos usar o get
    // Option<&i32> -> Isso está falando que vc terá ou um None ou um emprestimo de um int 32
    let emprestimo_v3: Option<&i32> = vec_3.get(2);
    println!("Peguei emprestado: {:?}", emprestimo_v3);
    match emprestimo_v3 {
        Some(valor) => println!("Valor que peguei emprestado: {}", valor),  //Some(valor) já cria a variavel valor
        None => println!("O emprestimo não existe.")
    }

    // iterar sem alterar
    for i in &vec_3 {
        println!("i: {i}")
    }
    let mut vec_5: Vec<String> = Vec::new();
    vec_5.push("X".to_string());
    vec_5.push("P".to_string());
    vec_5.push("T".to_string());
    vec_5.push("O".to_string());
    // iterando alterando
    for i in &mut vec_5 {
        i.push_str("_add");
        println!("i: {i}");
    }

    let mut vec_6: Vec<i32> = vec![1, 2, 3];
    for i in &mut vec_6 {
        *i += 100;  // quando você usa += é preciso usar o * para fazer o dereferencing
        println!("i: {i}");
    }

    // sort um vector
    let mut vec_7: Vec<i32> = vec![7, 8 , 6, 10, 9, 2, 6, 0];
    println!("vec_7: {:?}", vec_7);
    vec_7.sort_unstable();  // você pode usar apenas o sort(), perceba é que inplace
    println!("vec_7: {:?}", vec_7);

    // Remove o ultimo elemento
    let x: Option<i32> = vec_7.pop();
    if let Some(ultimo_valor) = x {
        println!("O ultimo valor existe e era: {}", ultimo_valor);
    }

    // retirando um elemento qualquer
    vec_7.remove(1);  // passa o index (removeu o valor 2)
    println!("vec_7: {:?}", vec_7);

    // Para problemas com FIFO (first in first out) use VecDeque
}