use std::time::Instant;
use rayon::prelude::*;

const N: u64 = 100_000_000;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u64;

    let mut i = 3;
    while i <= limit {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn sequential_worker(numbers: &[u64]) -> Vec<u64> {
    numbers
        .iter()
        .cloned()
        .filter(|&n| is_prime(n))
        .collect()
}

fn parallel_worker(numbers: &[u64]) -> Vec<u64> {
    numbers
        .par_iter()
        .cloned()
        .filter(|&n| is_prime(n))
        .collect()
}

fn main() {
    println!("Procurando números primos até {}...", N);

    let numbers: Vec<u64> = (0u64..N).collect();

    println!("Versão Sequencial...");
    let start = Instant::now();
    let primes = sequential_worker(&numbers);
    let time_seq = start.elapsed();
    println!("Encontrados {} primos", primes.len());
    println!("Tempo: {:.2?}", time_seq);

    println!("Versão Paralela...");
    let start = Instant::now();
    let primes = parallel_worker(&numbers);
    let time_parallel = start.elapsed();
    println!("Encontrados {} primos", primes.len());
    println!("Tempo: {:.2?}", time_parallel);

    let speedup = time_seq.as_secs_f64() / time_parallel.as_secs_f64();
    println!("\nSpeedup: {:.2}x", speedup);
}