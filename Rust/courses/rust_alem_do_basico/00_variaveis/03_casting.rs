fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}


fn main() {
    let inteiro: i32 = 10;
    let int_to_float: f32 = inteiro as f32;
    println!("Valor da variavel inteiro: {}, {}", inteiro, type_of(inteiro));
    println!("Valor da variavel int_to_float: {}, {}", int_to_float, type_of(int_to_float));

    let ponto_flutuante: f64 = 2.5;
    let float_to_int = ponto_flutuante as i32;
    println!("Valor da variavel float_to_int: {}, {}", float_to_int, type_of(float_to_int));

    let int_to_string = inteiro.to_string();  // vira uma String
    println!("Valor da variavel int_to_string: {}, {}", int_to_string, type_of(&int_to_string));

    let string: &str = "42";
    let string_to_int = string.parse::<i32>().unwrap();  // unwrap desempacota o valor
    println!("Valor da variavel string_to_int: {}, {}", string_to_int, type_of(&string_to_int));
}