## Five minutes Rust

Rust concise 5 Minutes cheat-sheet for each concepts.

---

## Hello, Rust!

```rust
fn main() {
    println!("Hi There!{}", "Ankur Anand");
}
```

**Important `println!` is not an function. It's a macro in rust. (macro end with `!`)**

**println! macro**

will accept an string, with `{}` as placeholder.

First string argument is **format strings**
with **{}** as **format specifiers**.

`{}` specifier for primitives types.

`{:?}` specifier for other types.

`rustc hello.rs`

Run the executables

---

## Variable Bindings

How to store a value and refer it later ?

In rust we can decalare a variable with `let` keyword. i.e Variables are bound with `let`

```rust
let name = "ankur";
```

- Bindings are implicitly-typed: the compiler infers based on context.
- The compiler can't always determine the type of a variable, so sometimes you have to add type annotations.

```rust
let x: i16 = 9;
```

> All the variables are **immutable** by **default** in Rust. i.e Once the variable has been decalared and initialized, you cannot assign some other value to it by default.

Trying to reassign will throw you an error at compile time.

```rust
fn main() {
    let x_immutable = 10;
    println!("x value : {}", x_immutable);
    x_immutable = 20;
}
```

```sh
error[E0384]: cannot assign twice to immutable variable `x_immutable`
 --> var.rs:4:5
  |
2 |     let x_immutable = 10;
  |         ----------- first assignment to `x_immutable`
3 |     println!("x value : {}", x_immutable);
4 |     x_immutable = 20;
  |     ^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
```

So if We want to reassign variable to some other value, add `mut` keyword before it.

```rust
fn main() {
    let mut y_mutable = 20;
    println!("y mut {}", y_mutable);
    y_mutable = 30;
    println!("y mut {}", y_mutable);
}
```

- Bindings may be shadowed:

```rust
let x = 17;
let y = 53;
let x = "Shadowed!";
// x is not mutable, but we're able to re-bind it
```

- The shadowed binding for `x` above lasts until it goes out of scope.
- Above, we've effectively lost the first binding, since both `x`s are in the same scope.

- Patterns may also be used to declare variables:

```rust
let (a, b) = ("foo", 12);
```

---
