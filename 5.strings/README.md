## Strings in Rust

To `&str` or `String`. ? (⊙.☉)7

1. Rust strings are valid UTF-8 encoded byte sequences.

2. `&` is an **operator**. It creates a pointer to any type.
That means `&str` types are usually a pointer to some existing string. What is the location of this existing string? It can be anywhere on *heap*, *stack* etc. It depends.

3. `String` types are allocated on **heap**.

How do we differentiate between these two types ?

Depend's upon initialization.

### `&str` type initialization
```rust
let amp_str_type = "amp_str_type";
```

### `String` type initialization.

```rust
let name: String = "ankur".to_string();
let last_name = String::from("anand");
```