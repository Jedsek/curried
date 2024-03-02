use curried::{curry, to_curry};
use std::fmt::Display;

#[curry]
fn add(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

#[curry]
fn concat_string<T>(a: T, b: T, c: T) -> String
where
    T: Display + 'static, // Note: You should additionally add 'static
{
    a.to_string() + &b.to_string() + &c.to_string()
}

fn map(a: i32, b: i32, c: i32) -> i32 {
    a + b - c
}

#[test]
fn normal_curry() {
    let i = add(1)(2)(3);
    assert_eq!(i, 6);
}

#[test]
fn generic_curry() {
    let f = concat_string(1)(23);
    let s = f(456);
    assert_eq!(s, "123456");
}

#[test]
fn map_curry() {
    let f = to_curry!(|a, b, c| map(b, a, c));
    let i = [1, 2, 3].map(f(1)(-3));
    assert_eq!(i, [-3, -4, -5]);
}
