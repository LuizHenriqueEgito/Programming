use std::collections::HashMap;

macro_rules! hashmap {
    ($($key:expr => $value: expr), * $(,)?) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

fn main() {
    let config = hashmap! {
        "host" => "localhost",
        "port" => "8080",
        "timeout" => "30",
    };

    println!("Config: {:?}", config);
}