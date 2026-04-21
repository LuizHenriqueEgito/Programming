/*
- Functions are called at runtime;
- Macros are expanded at compile time;
- Macros can take a vaiable number of arguments;
- Macros can generate ccode that functions can't;

O que tem ! em rust é uma maco, println!, format!, vec!, ..., etc
*/

macro_rules! answer {
    () => {
        42
    };
}

fn main() {
    let the_answer = answer!();
    println!("The answer is {}", the_answer);
}

