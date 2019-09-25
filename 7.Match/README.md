## Match expressions

**switch on steroids**

- match takes an expression (x) and branches on a list of value => expression statements. so all the expressions within the `match` block is known as **match arms**

```rust
fn http_status() -> u16 {
    500
}

fn main() {
    let status = http_status();
    match status {
        // all the expressions
        500 => println!("Internal Server Error Status {}", status),
        200 => println!("Yay Got [{}]", status),
        _ => println!("ingnore value {}", status),
    }
}
```

- Block after `=>` are also expressions in rust and must return the same type. When nothing is returned by default an Unit Type `()` is returned. i.e

  > The entire match evaluates to one expression. Like if, all arms must evaluate to the same type.

- `_` is commonly used as a catch-all

- The matched expression can be any expression (l-value), including tuples and function calls.

```rust
fn main() {

    let x = 3;
    let y = -3;

    match (x, y) {
        (1, 1) => println!("one"),
        (2, j) => println!("two, {}", j),
        (_, 3) => println!("three"),
        (i, j) if i > 5 && j < 0 => println!("On guard!"),
        (_, _) => println!(":<"),
    }
}
```

- Matches can bind variables. `_` is a throw-away variable name.
- it should be an exhaustive match in order to compile.
- if-guards to constrain a match to certain conditions.
