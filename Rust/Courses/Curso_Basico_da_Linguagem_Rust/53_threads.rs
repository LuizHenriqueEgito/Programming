use std::thread;
use std::time::Duration;


fn thread_tamanho(id: &usize, s: &String) -> usize {
    println!("Thread filha {id} - Recebeu {s} com tamanho {}", s.len());
    thread::sleep(Duration::from_millis(1000));
    s.len()
}

fn main() {
    let dados = vec![
        "AAAAA".to_string(),
        "BBBBB".to_string(),
        "CCCCC".to_string(),
        "DDDDD".to_string(),
    ];
    let ids = vec![0, 1, 2, 3];

    // retorna total
    let ret_scope = thread::scope(
        | scope | {
            let mut handles = Vec::new();
            // cria n threads
            for i in 0..dados.len() {
                let id = &ids[i];
                let s = &dados[i];
                handles.push(
                    scope.spawn(move || {
                        thread_tamanho(id, s)
                    })
                );
            }

            // Espera todas as threads filhas terminarem
            let mut total = 0;
            for h in handles.into_iter() {
                if let Ok(x) = h.join() {
                    total += x;
                }
            }
            total
        }
    );
    println!("Thread main: Scope retornou: {}", ret_scope);

    for i in 1..5 {
        println!("Thread main: Está na contagem{i}");
        thread::sleep(Duration::from_millis(100));
    }
    println!("Dados: {:?}", dados);
    println!("Thread main: Terminou!")
}