enum DataSize {
    Byte,
    KB,
    MB,
    GB,
}

fn bytes(size: DataSize) -> u64 {
    match size {
        DataSize::Byte => {
            println!("1 byte is 1 byte, mate...");  // podemos fazer isso
            1
        },
        DataSize::KB => 1000,
        DataSize::MB => 1000 * 1000,
        DataSize::GB => 1000 * 1000 * 1000,
    }
}

fn main() {
    let kb = bytes(DataSize::KB);
    println!("kb={:?}", kb);
}