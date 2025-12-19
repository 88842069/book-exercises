// rewriting the guessing game myself
// pseudocode:
// DONE 1. generate a secret number
// DONE 2. take input from user
// DONE 3. validate the input
// DONE 4. compare the input
// DONE 5. give feedback to user (guess is too big or small)
// DONE 6. if their guess is correct, say so and quit the program

use rand::{Rng, thread_rng};
use std::io;

fn main() {
    let secret = thread_rng().gen_range(0..10);

    println!("welcome to the guessing game!");
    println!("we have selected a number from 1 to 10");

    loop {
        let mut guess = String::new();

        println!("\nwhat's the number? ");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("WARN: please only enter numbers!");
                continue;
            }
        };

        if guess > secret {
            println!("your guess {} is higher!", guess);
        } else if guess < secret {
            println!("your guess {} is lower!", guess);
        } else {
            println!("that's right!");
            break;
        }
    }
}
