## How to store a value and refer it later ?

In rust we can decalare a variable with `let` keyword.

```rust
fn main() {
    let x_immutable = 10;
    println!("x value : {}", x_immutable);
}
```

```go
var v int
v = 2
v = 3
```
The above go code where you can reassign v as many times as you wish.

**Rust default variable declaration doesn't allow you to do this**

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