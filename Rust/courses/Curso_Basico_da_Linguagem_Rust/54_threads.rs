/*
Como duas threads distintas podem usar o mesmo canal?
Elas revcebem o canal no momento da criação da thread.

Paralelismo real uma thread processa e envia para outra que faz o
seu processamento.
*/

use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

fn thread_que_envia(tx: Sender<String>) {
    let msg = "Mensagem secreta".to_string();
    tx.send(msg).unwrap();
}

fn thread_que_recebe(rx: Receiver<String>) {
    if let Ok(recebida) = rx.recv() {
        println!("thread_que_recebe: Mensagem recebida: {}", recebida);
    } else {
        println!("thread_que_recebe: Este canal não tem mais remetentes ativos!")
    }

    thread::sleep(Duration::from_secs(3));

    if let Ok(recebida) = rx.recv() {
        println!("thread_que_recebe: Mensagem recebida: {}", recebida);
    } else {
        println!("thread_que_recebe: Este canal não tem mais remetentes ativos!")
    }
}

fn main() {
    // Cria o canal
    let (tx, rx) = mpsc::channel();

    // Cria a thread que envia
    let handle_1 = thread::spawn(move || {
        thread_que_envia(tx)
    });

    // Cria thread que recebe
    let handle_2 = thread::spawn(move || {
        thread_que_recebe(rx)
    });

    // Espera ambas terminarem
    _ = handle_1.join();
    _ = handle_2.join();
    println!("Thread main: Terminou");
}
