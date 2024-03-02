# curried

## Intro

Currying function to be used in normal && generic && map case, with procedural macro.  
(This crate could be used in stable channel.)

## Usage

```rust
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
    let f = to_curry!(|a, b, c| map(b, a, c));
    let i = [1, 2, 3].map(f(1)(-3));
    assert_eq!(i, [-3, -4, -5]);
}
```

## Note

- These code could be successfully compiled:
additional
```rust
fn f<T>(_: i32, _: T) {}
f(1, 1);
f(1, "123");


#[curry]
fn g<T>(_: i32, _: T) {}
g(1)(1);
g(1)("123");
```

- But these would not:

```rust
#[curry]
fn f<T>(_: i32, _: T) {}

let g = f(1);
g(1);
g("123");
```

- If you want to use the same function with different typed argument:

```rust
fn f<T>(_: i32, _: T) {}

let g1 = to_curry!(|a, b| f(a, b));
let g2 = to_curry!(|a, b| f(a, b));

let gg1 = g1(1);
gg1(1);

let gg2 = g2(1);
gg2("123");
```

- If you want to use curried function in `map`:

```rust
// Don't use [curry] proc_attr_macro, use to_curry! to auto deduce type for closure type
let f = to_curry!(|a, b, c| product(a, b, c));  

[1, 2, 3].map(f(10)(10));  // [100, 200, 300]
```

- `to_curry!` could change the order of passed-in arguments:

```rust
fn concat_into_string(a: i32, b: i32, c: i32) -> String {
  unimplemented!();
}

let f1 = to_curry!(|a, b, c| concat_into_string(a, b ,c));
f1(1)(23)(456)  // "123456"

let f2 = to_curry!(|a, b, c| concat_into_string(b, c ,a));
f2(1)(23)(456)  // "234561"
```

