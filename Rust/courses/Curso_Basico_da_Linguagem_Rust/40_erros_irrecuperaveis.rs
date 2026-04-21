fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("vector: {:?}", v);
    let y: usize = 100;
    if y > v.len() {
        panic!("Entrei em pânico!")  // Panico: O compilador não sabe o que fazer!
    }
    let x: i32 = v[y];  // isso da panic! pois seu vetor não possui 100 elementos
    /*
    thread 'main' panicked at 40_erros_irrecuperaveis.rs:8:19:
    index out of bounds: the len is 3 but the index is 100
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */
}