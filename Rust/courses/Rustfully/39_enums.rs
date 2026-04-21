enum State {
    On,
    Off,
}

enum IP {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let on = State::On;
    let off  = State::Off;

    let home = IP::V4(127, 0, 0, 1);
    let loopback = IP::V6(String::from("::1"));
}