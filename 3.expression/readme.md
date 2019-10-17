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
