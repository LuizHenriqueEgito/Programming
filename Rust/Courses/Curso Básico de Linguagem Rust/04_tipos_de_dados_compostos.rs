fn main() {
    // tupla
    let tup1: (i32, f64, bool) = (500, 6.4, true);
    let tup2: (i32, f64, bool) = (500, 6.4, true);
    println!("Minha tupla tem: {:?}", tup2);  // minha tupla não pode ser printada eu preciso implementar std::fmt::Display
    // no entando ele pode fazer algo por mim e printar da maneira que ele achar melhor por isso usar o {:?} ou o {:#?}
    println!("Minha tupla tem: {:#?}", tup1);
    let (x1, y1, z1) = tup2;
    println!("Minha tupla tem: {x1} {y1} {z1}");
    // posso acessar os elementos da tupla pelo seu index
    println!("Minha tupla tem: {:?} {:?} {:?}", tup1.0, tup1.1, tup1.2);
    // uma tupla vazia é chamada de unit, representa um valor vazio
    println!("Tupla vazia: {:?}", ());

    // array (Todos os elementos precisam ser do mesmo tipo)
    let aa_: [i32; 5] = [1, 2, 3, 4, 5];
    let meses: [&str; 3] = ["Jan", "Fev", "Mar"];

    let bb_: [i32; 5] = [6, 7, 8, 9, 10];

    let cc: [i32; 5] = [3; 5]; // repete o inteiro 3 cinco vezes
    let dd: [i32; 2] = [3, 5];

    println!("cc {:?}", cc);
    println!("dd {:?}", dd);

    println!("Elemento 2 do array 'meses' é: {:#?}", meses[2])
}