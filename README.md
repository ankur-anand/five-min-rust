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

---

## Macros!

- Macros are like functions, but they're named with `!` at the end.
- Can do generally very powerful stuff.
  - They actually generate code at compile time!
  - Think macro in `c` but more hygienic. More later!
- Call and use macros like functions.
- You can define your own with `macro_rules! macro_name` blocks.
  - These are _very_ complicated. More later!
- Because they're so powerful, a lot of common utilities are defined as macros.
- **println! macro**

### `print!` & `println!`

- Print stuff out. Yay.
- Use `{}` for general string interpolation, and `{:?}` for debug printing.
  - Some types can only be printed with `{:?}`, like arrays and `Vec`s.

```rust
print!("{}, {}, {}", "foo", 3, true);
// => foo, 3, true
println!("{:?}, {:?}", "foo", [1, 2, 3]);
// => "foo", [1, 2, 3]
```

---

### `format!`

- Uses `println!`-style string interpolation to create formatted `String`s.

```rust
let fmted = format!("{}, {:x}, {:?}", 12, 155, Some("Hello"));
// fmted == "12, 9b, Some("Hello")"
```

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

## Expressions

**Expression are something that returns a value**

- Almost everything is an expression: in Rust. Except _variable bindings_

- The "nothing" type is called "unit", which is written `()`.
  - The _type_ `()` has only one value: `()`.
  - `()` is the default return type.
- Discard an expression's value by appending a semicolon. Now it returns `()`.
  - Hence, if a function ends in a semicolon, it returns `()`.

```rust
fn foo() -> i32 { 5 }
fn bar() -> () { () }
fn baz() -> () { 5; }
fn qux()       { 5; }
```

- Because everything is an expression, we can bind many things to variable names:

```rust
let x = -5;
let y = if x > 0 { "greater" } else { "less" };
println!("x = {} is {} than zero", x, y);
```

- Aside: `"{}"` is Rust's (most basic) string interpolation operator
  - like `printf`'s `"%s"` in C/C++.
