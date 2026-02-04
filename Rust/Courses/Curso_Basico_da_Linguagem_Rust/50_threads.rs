// Threads são fluxos de execução concorrentes (paralelismo físico ou virtual)
use std::thread;
use std::time::Duration;

fn main() {
    // Cria uma thread filha
    thread::spawn(|| {
        for i in 1..10 {
            println!("Thread filha está na contagem {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("Thread main está na contagem {}", i);
        thread::sleep(Duration::from_secs(1));
    }

    println!("Thread main terminou!")
}