use testing::ops::*;

mod common;

#[test]
fn subtract_and_divide() {
    let a = 20;
    let b = 2;
    let c = 3;

    let difference = sub(&a, &b);
    let quotient = div(&difference, &c);
    assert_eq!(quotient, 6);
}
#[test]
fn add_and_multiply() {
    let a = 10;
    let b = common::give_me_5() as i32;
    let c = 3;

    let sum = add(&a, &b);
    let product = mul(&sum, &c);
    assert_eq!(product, 45);
}
