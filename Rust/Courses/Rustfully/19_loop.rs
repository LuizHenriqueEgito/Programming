// loop regular

fn main() {
    let mut counter = 0;
    let result = loop {
        println!("Count: {counter}");
        counter += 1;

        if counter == 5 {
            println!("We reached 0!");
            break "Sucess";  // valor retornado
        }
    };
    println!("result={:?}", result);
}