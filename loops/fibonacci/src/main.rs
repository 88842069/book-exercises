use std::io;

fn fib(x: i32) -> i32 {
    if x == 1 {
        return 0;
    } else if x == 2 {
        return 1;
    } else {
        return fib(x - 1) + fib(x - 2);
    }
}

fn main() {
    println!("welcome to the fibo party!");
    println!("which fibonacci number would you like to see?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error storing user input");

    let input: i32 = input
        .trim()
        .parse()
        .expect("error converting to number: did you enter a non-number?");

    println!("fibonacci number {input} is {}", fib(input));
}
