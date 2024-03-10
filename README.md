# curried

## Intro

Currying function to be used in normal && generic && map case, with procedural macro.  
(This crate could be used in stable channel.)

## Usage

```rust
use curried::{curry, to_curry};

#[curry]
fn add(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

#[curry]
fn concat_string<T>(a: T, b: T, c: T) -> String
where
    T: std::fmt::Display + 'static, // Note: You should additionally add 'static
{
    a.to_string() + &b.to_string() + &c.to_string()
}

fn map(a: i32, b: i32, c: i32) -> i32 {
    a - b - c
}

fn normal_curry() {
    let i = add(1)(2)(3);
    assert_eq!(i, 6);
}

fn generic_curry() {
    let f = concat_string(1)(23);
    let s = f(456);
    assert_eq!(s, "123456");
}

fn map_curry() {
    let f = to_curry!(map(a, b, c));
    let i = [1, 2, 3].map(f(1)(2));
    assert_eq!(i, [-2, -3, -4]);
}
```

## Note

- These code could be successfully compiled:

```rust
fn f<T>(_: i32, _: T) {}
f(1, 1);
f(1, "123");


#[curried::curry]
fn g<T>(_: i32, _: T) {}
g(1)(1);
g(1)("123");
```

- But these would not:

```compile_failed
#[curried::curry]
fn f<T>(_: i32, _: T) {}

let g = f(1);
g(1);
g("123");
```

- If you want to use the same function with different typed argument:

```rust
use curried::to_curry;

fn f<T>(_: i32, _: T) {}

let g1 = to_curry!(f(a, b));
let g2 = to_curry!(f(a, b));

let gg1 = g1(1);
gg1(1);

let gg2 = g2(1);
gg2("123");
```

- If you want to use curried function in `map`:

```rust
use curried::to_curry;

fn product(a: i32, b: i32, c: i32) -> i32 {
    a * b * c
}

// Don't use [curry] proc_attr_macro, use to_curry! to auto deduce type for closure type
let f = to_curry!(product(a, b, c));  

[1, 2, 3].map(f(10)(10));  // [100, 200, 300]
```
