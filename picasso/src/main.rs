// picasso: take 3 rectangles from user
// if none of the 3 rectangles fit in one another
// user wins: picasso!
// else, inform user that they lost and how

use std::io;

#[derive(Debug)]
struct Rectangle {
    w: i32,
    h: i32,
}

fn build() -> Rectangle {
    let mut w = String::new();
    let mut h = String::new();

    println!("\tplease enter width:");

    io::stdin()
        .read_line(&mut w)
        .expect("failed to read user input");

    let w: i32 = match w.trim().parse() {
        Ok(num) => num,
        Err(_) => todo!(),
    };

    println!("\tplease enter height:");

    io::stdin()
        .read_line(&mut h)
        .expect("failed to read user input");

    let h: i32 = match h.trim().parse() {
        Ok(num) => num,
        Err(_) => todo!(),
    };

    Rectangle { w, h }
}

fn main() {
    print_rules();

    println!("\nenter details of rectangle 1:");
    let r1: Rectangle = build();

    println!("\nenter details of rectangle 2:");
    let r2: Rectangle = build();

    println!("\nenter details of rectangle 3:");
    let r3: Rectangle = build();

    let result: [bool; 6] = compare(&r1, &r2, &r3);

    print_result(&result);
}
fn compare(r1: &Rectangle, r2: &Rectangle, r3: &Rectangle) -> [bool; 6] {
    let mut result: [bool; 6] = [false; 6];

    // r2 can fit in r1
    if r1.w > r2.w && r1.h > r2.h {
        result[0] = true;
    };
    // r1 can fit in r2
    if r1.w < r2.w && r1.h < r2.h {
        result[1] = true;
    };

    // r3 in r2
    if r2.w > r3.w && r2.h > r3.h {
        result[2] = true;
    };
    // r2 can fit in r3
    if r2.w < r3.w && r2.h < r3.h {
        result[3] = true;
    };

    // r3 in r1
    if r1.w > r3.w && r1.h > r3.h {
        result[4] = true;
    };
    //r1 can fit in r3
    if r1.w < r3.w && r1.h < r3.h {
        result[5] = true;
    };

    result
}

fn print_rules() {
    println!("welcome to picasso!");
    println!("\nthe rules are simple:");
    println!("\n\tenter dimensions of 3 rectangles such that");
    println!("\n\tnone of the rectangles can fit in any other");
    println!("\n\tif any of the rectangles fit in any other one, you lose!");
    println!("\nReady?");
}

fn print_result(result: &[bool; 6]) {
    if *result == [false; 6] {
        println!("\n\t-----picasso!-----");
        return;
    }

    if result[0] == true {
        println!("...r2 can fit in r1");
    };
    if result[1] == true {
        println!("...r1 can fit in r2");
    };
    if result[2] == true {
        println!("...r3 can fit in r2");
    };
    if result[3] == true {
        println!("...r2 can fit in r3");
    };
    if result[4] == true {
        println!("...r3 can fit in r1");
    };
    if result[5] == true {
        println!("...r1 can fit in r3");
    };
}

// i see a lot of improvements
// first, we can use a struct to store the results
// iterate over the struct, if anything is true then print the struct name
// make the build function more idiomatic (chapter 13 has a nice build function we can borrow from)
// add tests for all 6 cases
// show the user a simulation of the rectangles!
