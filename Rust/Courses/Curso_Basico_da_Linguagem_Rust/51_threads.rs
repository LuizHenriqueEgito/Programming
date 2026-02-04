use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread filha está na contagem {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();  // Espera a thread filha terminar (serializa a execução)
    println!("Thread filha finalizou.");
    for i in 1..5 {
        println!("Thread main está na contagem {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("Thread main finalizou.");
}