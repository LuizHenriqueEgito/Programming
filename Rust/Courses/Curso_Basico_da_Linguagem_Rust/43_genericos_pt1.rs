#[derive(Debug)]
struct UmaStruct {
    i1: i32,
    i2: i32,
}

// <T>: Lida com tipos de dados que eu nem sei quais serão
fn get_first_element<T>(list: &[T]) -> &T {
    &list[0]
}

/*
Aqui básicamente eu estou falando que não aceito qualquer tipo generico T
Eu só aceito aqueles que impplementem std::cmp::PartialOrd, ou seja os que
aceitem ordenação
*/
fn get_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn main() {
    let array_int: [i32; 5] = [1, 2, 3, 4, 5];
    let vector: Vec<i32> = vec![1, 2, 3, 4];
    
    let array: [UmaStruct; 3] = [
        UmaStruct{i1: 1, i2: 1},
        UmaStruct{i1: 2, i2: 3},
        UmaStruct{i1: 4, i2: 5},
    ];
    println!("UmaStruct: {:#?}", array);
    println!("Valor pego: {}", get_first_element(&array_int));
    println!("Valor pego: {}", get_first_element(&vector));
    println!("Valor pego: {:#?}", get_first_element(&array));
    println!("O maior valor: {}", get_largest(&array_int));
}