#[derive(Debug)]
enum TipoIP {
    V4,
    V6,
}

#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrDif {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let tipo1 = TipoIP::V4;
    let tipo2;
    tipo2 = TipoIP::V6;
    println!("tipo1={:?} | tipo2={:?}", tipo1, tipo2);

    let home = IPAddr::V4(String::from("127.0.0.1"));
    let loopback = IPAddr::V6(String::from("::1"));
    println!("home={:?} | loopback={:?}", home, loopback);

    let home = IpAddrDif::V4(127, 0, 0, 1);
    let loopback = IpAddrDif::V6(String::from("::1"));
    println!("home={:?} | loopback={:?}", home, loopback);
}