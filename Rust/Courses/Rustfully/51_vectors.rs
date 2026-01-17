use std::f64::consts::PI;

#[derive(Debug)]
enum Value {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let v: Vec<i32> = Vec::new();
    println!(v)
    let v: Vec<i32> = vec![1, 2, 3];
    println!("v={:?}", v);
    let mut numbers: Vec<i32> = vec![0];
    numbers.push(1);
    numbers.push(2);
    println!("numbers={:?}", numbers);
    numbers.pop()  // como o pop no python -> retorna o valor excluido
    let second: Option<&i32> = numbers.get(1);
    match second {
        Some(second) => println!("The second element is: {second}."),
        None => println!("Thre is no second element.")
    }
    let people = vec!["Bob", "James", "Sandra"];
    for person in &people {
        println!("{person}.");
    }

    let mut values: Vec<Value> = vec!{Value::Float(PI), Value::Int(42)};
    values.push(Value::Text(String::from("Bob")));
    println!("values={:?}", values);
}
