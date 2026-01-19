use testing::print_operations;
use testing::users::*;

fn main() {
    let a = 14;
    let b = 12;
    let user = "admin";
    let john = "john";

    print_operations(&a, &b);

    if admin_checker(user) {
        println!("you have the relevant permissions");
    };

    if john_checker(john).unwrap() {
        println!("hi john!");
    };
}
