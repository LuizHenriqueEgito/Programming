const QTY: f64 = 10.0;
const OZ: f64 = 5.5


fn do_stuff(qty: f64, oz: f64) -> f64 {
    return qty * oz;  // você pode fazer isso sem o return apenas com qty * oz
}

fn main() {
    value = do_stuff(QTY, OZ);
    println!("value: {}", value);
}