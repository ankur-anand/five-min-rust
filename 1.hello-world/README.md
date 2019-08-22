1. Hello World

```rust
fn main() {
    println!("Hi There!{}", "Ankur Anand");
}
```

compile the code
`rustc hello.rs`

Run the executables
`./hello` on linux

**Important `println!` is not an function. It's a macro in rust. (macro end with `!`)**

### println! macro
will accept an string, with `{}` as placeholder.

First string argument is **format strings**
with **{}** as **format specifiers**.

#### `{}` specifier for primitives types.
#### `{:?}` specifier for other types.

## Advanced (Skip if first time)

When an **format specifier** founds it's corresponding substitue value, it calls a `method` on that value, which return a string format of that value. (More on this later)