fn main() {
    let mut main_count = 0;

    // você da um lifetime para o loop, é como um nome um tempo de vida
    'main: loop {
        println!("Outer: {main_count}");
        let mut inner_count = 0;

        'inner: loop {
            println!("Inner: {inner_count}");
            inner_count += 1;

            if inner_count == 3 {
                println!("---");
                break 'inner;
            }

            if main_count == 3 {
                println!("Exiting out of all loops");
                break 'main;  // para aquele tempo de vida do loop anterior você consegue sair dele e encerar seu lifetime
            }
        }
        main_count += 1
    }
}