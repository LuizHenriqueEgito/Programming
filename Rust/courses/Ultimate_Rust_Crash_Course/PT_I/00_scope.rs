fn main() {
    let x = 5;
    let z = 10;
    {
        let y = 99;
        let z = 111
        println!("Neste escopo x e y existem:")
        println!("x={}, y={}, z={}", x, y, z);
    }
    println!("Aqui só existe x={} e z={}", x, z);
}