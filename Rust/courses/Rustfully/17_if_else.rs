fn amin() {
    let n = 11;
    let odd_even = if n % 2 == 0 {"Even"} else {"Odd"};  // operador ternário
    println!(odd_even);

    let is_on = false;
    let result = if is_on {0} else {-1};
    println!(result)
}