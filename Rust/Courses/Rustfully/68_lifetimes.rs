/*
lifetimes são outro tipo de genericos
os tempos de vida garantem que as referencias sejam validas pelo tempo que precisamos
*/


// Isso não compila:
// fn main() {
//     let r;  // 'a: tempo de vida de r
//     {
//         let x = 5;  // 'b: tempo de vida de x
//         r = &x;  
//     }  // -- final do tempo de vida de x
//     println!("r: {r}");
// }  // -- 'a: final do tempo de vida de r

fn main() {
    let x = 5;  // 'b
    let r = &x;  // 'a

    println!("r: {r}");
}