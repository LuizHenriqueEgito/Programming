use std::thread;
use std::time::Duration;

fn main() {
    // é como se ao final tivessemos um join de todos
    thread::scope(
        | scope | {
            scope.spawn(
                || {
                    println!("Thread ONE iniciou");
                    thread::sleep(Duration::from_secs(2));
                    println!("Thread ONE terminou");
                }
            );
            scope.spawn(
                || {
                    println!("Thread TWO iniciou");
                    thread::sleep(Duration::from_secs(2));
                    println!("Thread TWO terminou");
                    // se ela panicar quebra o programa
                    // panic!("Panica thread TWO"); use let handler = e ao final do escope dê join nele
                }
            );
            scope.spawn(
                || {
                    println!("Thread THREE iniciou");
                    thread::sleep(Duration::from_secs(2));
                    println!("Thread THREE terminou");
                }
            );
        }
    );
    println!("Thread main terminou.")
}