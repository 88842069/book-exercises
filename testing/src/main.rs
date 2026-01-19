use crate::users::{admin_checker, john_checker};

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

fn print_operations(a: &i32, b: &i32) {
    let sum = ops::add(&a, &b);
    let difference = ops::sub(&a, &b);
    let product = ops::mul(&a, &b);
    let quotient = ops::div(&a, &b);

    println!(
        "
        a is {a} and b is {b} and 
        sum        = {sum}
        difference = {difference}
        product    = {product}
        quotient   = {quotient}
        "
    );
}

mod ops {
    pub fn add(a: &i32, b: &i32) -> i32 {
        *a + *b
    }
    pub fn sub(a: &i32, b: &i32) -> i32 {
        *a - *b
    }
    pub fn mul(a: &i32, b: &i32) -> i32 {
        *a * *b
    }
    pub fn div(a: &i32, b: &i32) -> i32 {
        *a / *b
    }
}

mod users {

    pub fn admin_checker(user: &str) -> bool {
        if *user == "admin".to_string() {
            true
        } else {
            panic!("bruh moment");
        }
    }

    pub fn john_checker(user: &str) -> Result<bool, String> {
        if *user == "john".to_string() {
            Ok(true)
        } else {
            Err("panicking because you're not john!".to_string())
        }
    }
}

#[cfg(test)]
mod panic_tests {

    use crate::users::{admin_checker, john_checker};

    #[test]
    #[should_panic(expected = "bruh")]
    fn is_admin() {
        let user = "notadmin";
        admin_checker(user);
    }

    #[test]
    fn is_john() -> Result<(), String> {
        let user = "john";
        let result = john_checker(user);

        match result {
            std::result::Result::Ok(true) => Ok(()),
            std::result::Result::Ok(false) => {
                Err("is_john test failed because it's someone else!".to_string())
            }
            Err(_) => Err("is_john test failed because it's someone else!".to_string()),
        }
    }
}

#[cfg(test)]
mod tests_returning_results {
    use crate::ops::*;

    #[test]
    fn test_add() -> Result<(), String> {
        let a = 12;
        let b = 14;
        let result = add(&a, &b);
        if result == 26 {
            Ok(())
        } else {
            Err("12 + 14 should be 26 but it's not".to_string())
        }
    }

    #[test]
    fn test_sub() -> Result<(), String> {
        let a = 12;
        let b = 14;
        let result = sub(&a, &b);

        if result == -2 {
            Ok(())
        } else {
            Err("error calculating difference".to_string())
        }
    }

    #[test]
    #[ignore]
    fn test_mul() -> Result<(), String> {
        let a = 14;
        let b = 12;
        let result = mul(&a, &b);

        if result == 168 {
            Ok(())
        } else {
            Err("error calculating product".to_string())
        }
    }

    #[test]
    #[ignore]
    fn test_div() -> Result<(), String> {
        let a = 14;
        let b = 12;
        let result = div(&a, &b);

        if result == 1 {
            Ok(())
        } else {
            Err("error calculating quotient".to_string())
        }
    }
}

#[cfg(test)]
mod tests_returning_bool {

    use crate::ops::*;

    #[test]
    fn test_add() {
        let a = 12;
        let b = 14;
        let result = add(&a, &b);
        assert_eq!(result, a + b, "12 + 14 should be 26 but it's not");
    }

    #[test]
    fn test_sub() {
        let a = 12;
        let b = 14;
        let result = sub(&a, &b);
        assert_eq!(result, a - b);
    }

    #[test]
    #[ignore]
    fn test_mul() {
        let a = 12;
        let b = 14;
        let result = mul(&a, &b);
        assert_eq!(result, a * b);
    }

    #[test]
    #[ignore]
    fn test_div() {
        let a = 12;
        let b = 14;
        let result = div(&a, &b);
        assert_eq!(result, a / b);
    }
}
