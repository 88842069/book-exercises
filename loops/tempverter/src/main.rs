use std::io;

fn main() {
    println!("welcome to tempverter!");
    println!("please enter your temp:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error storing user input");

    let input: i32 = input
        .trim()
        .parse()
        .expect("error storing temperature - did you enter a number?");

    println!("DEBUG: you entered {input}");

    println!("is this in C or F?");

    let mut input_unit = String::new();

    io::stdin()
        .read_line(&mut input_unit)
        .expect("error storing user input");

    let input_unit: char = input_unit
        .trim()
        .parse()
        //NOTE: it won't panic when you enter a number
        //because char is an unsigned 32-bit integer
        //so it will akshually panic when you enter a negative number
        .expect("error storing temperature unit - did you enter a char?");

    if input_unit == 'c' || input_unit == 'C' {
        println!("{input}C is {}F", converter(input, 1));
    } else if input_unit == 'f' || input_unit == 'F' {
        println!("{input}F is {}C", converter(input, 2));
    }
}

fn converter(input: i32, choice: i32) -> i32 {
    if choice == 1 {
        return 32 + ((9 * input) / 5);
    } else {
        return ((input - 32) / 9) * 5;
    }
}
