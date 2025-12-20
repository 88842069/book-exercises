fn main() {
    let fac: i32 = factorial(5);
    println!("{fac}");
}

fn factorial(x: i32) -> i32 {
    let mut rev = x - 1;
    let mut result = x;
    loop {
        result *= rev;
        rev -= 1;

        if rev == 1 {
            return result;
        }
    }
}
