use rand::{Rng, rng};
use std::io;

enum SpinResult {
    Ace(u32),
    King(u32),
    Queen(u32),
    Jack(u32),
    Joker(u32),
}

fn main() {
    print_rules();
    let tries = manage_balance(load_money());

    let mut total_payout: u32 = 0;

    for _ in 0..tries {
        let outcome: SpinResult = spin();

        match outcome {
            SpinResult::Ace(num) => {
                println!("\tAce! +${num}");
                total_payout += num;
            }
            SpinResult::King(num) => {
                println!("\tKing! +${num}");
                total_payout += num;
            }
            SpinResult::Queen(num) => {
                println!("\tQueen! +${num}");
                total_payout += num;
            }
            SpinResult::Jack(num) => {
                println!("\tJack! +${num}");
                total_payout += num;
            }
            SpinResult::Joker(num) => {
                println!("\tJoker! +${num}");
                total_payout += num;
            }
        }
    }
    println!("\nyou walk away with ${total_payout}");
}

fn spin() -> SpinResult {
    let random: u32 = rng().random_range(0..5);
    match random {
        0 => SpinResult::Ace(10),
        1 => SpinResult::King(5),
        2 => SpinResult::Queen(2),
        3 => SpinResult::Jack(1),
        4 => SpinResult::Joker(0),
        _ => todo!(),
    }
}

fn load_money() -> (bool, u32, u32, u32) {
    let mut user_input_amount = String::new();

    io::stdin()
        .read_line(&mut user_input_amount)
        .expect("failed to read user input!");

    let user_input_amount: u32 = match user_input_amount.trim().parse() {
        Ok(num) => num,
        Err(_) => todo!(),
    };

    let tries: u32 = user_input_amount / 4;
    let change: u32 = user_input_amount % 4;

    if user_input_amount >= 12 {
        (true, tries, change, user_input_amount)
    } else {
        (false, tries, change, user_input_amount)
    }
}

fn manage_balance(user_input: (bool, u32, u32, u32)) -> u32 {
    if !user_input.0 {
        println!("please try again with $12 or more");
    } else {
        if user_input.2 > 0 {
            println!("\nhere's your balance: ${}", user_input.2);
        };
        println!("you have {} spins...", user_input.1);
    }

    user_input.1
}

fn print_rules() {
    println!("welcome to atlantic city!");
    println!("here are the slot machine rules:");
    println!("  1. $4 per spin");
    println!("  2. you must play a minimum of 3 spins");
    println!("  3. each spin can have 1 of 5 results:");
    println!("    * Ace - $10");
    println!("    * King - $5");
    println!("    * Queen - $2");
    println!("    * Jack - $1");
    println!("    * Joker - $0");
    println!("Please enter at least $12...");
}
