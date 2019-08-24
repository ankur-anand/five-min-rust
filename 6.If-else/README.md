## If else

### `if` `else` is not an Statement in Rust. It's an expression.

**So it always returns an value**

```rust
fn main() {
   let is_ture = true;
    if is_ture {
        println!("isTure");
    } else {
        println!("isFalse");
    }
}
```

So the above `if` and `else` will evaluate the condition while executing all instruction contained inside block evaluating to true and return an `empty () unit type`

**Both `if` `else` branch should return the same type**

```rust
fn main() {
    let is_ture = true;
   let if_else_result = if is_ture {
        "Hello isTure"
    } else {
        "Hello isFalse"
    };
    println!("{}", if_else_result);
}
```
