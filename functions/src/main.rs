// fn main() {
//     print_labelled_measurements(5, 'h');
// }
//
// fn print_labelled_measurements(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }
//
// // a variable
//
use std::io::{self};

fn main() {
    println!(
        "welcome to our rudimentary calculator!\nwe can do two things:\n1.factorial.2.square\nwhich one do you want?\n\t"
    );
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("couldn't read line!");

        let input: i32 = input.trim().parse().expect("numbers only");

        if input > 2 || input < 0 {
            println!("aye choose from the options foo");
            break;
        }

        println!("\nalright and what number to operate on?\n\t");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("couldn't read line!");

        let number: i32 = number
            .trim()
            .parse()
            .expect("aye foo enter a numberrr quit playin");

        if input == 1 {
            println!("\nthe factorial of {number} is {}", factorial(number));
            break;
        } else {
            println!("\nthe square of {number} is {}", square(number));
            break;
        }
    }

    fn square(x: i32) -> i32 {
        return x * x;
    }

    //TODO: fix factorial function after you figure out loops
    //NOTE: we have implemented factorial in the the loops sub-directory
    fn factorial(x: i32) -> i32 {
        let mut rev = x - 1;
        let mut result = x;
        loop {
            result *= rev;
            rev -= 1;

            if rev < 3 {
                break result * 2;
            }
        }
    }
}
