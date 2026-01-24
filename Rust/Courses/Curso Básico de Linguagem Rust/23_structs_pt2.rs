
/*
#[derive(Debug)] básicamente fala:
"Compilador implemente automaticamente o trait Debug para minha struct"
*/
// tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);  // RGB

#[derive(Debug)]
struct Point(i32, i32, i32);  // É como um namedtuple do Python

#[derive(Debug)]
struct AlwaysEqual;  // struct vazia


fn main() {
    let black: Color = Color(0, 0, 0);
    let origem: Point = Point(1, 2, 3);
    println!("Minha cor: {:?} | Meu ponto: {:?}", black, origem);
    let always_equal = AlwaysEqual;
    println!("Struct vazia: {:?}", always_equal);
}