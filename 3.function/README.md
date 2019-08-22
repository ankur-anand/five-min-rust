## function

In Rust we create a function starts with keyword `fn`.

```rust
fn add(x: u64, y: u64) -> u64 {
    return x + y;
}
```

types of each passed parameters are mentioned on the right side after `:` inside `()`.

`return` type is specified using `->`, followed by type name. Omit this if nothing gets returned.
**If nothing is returned an `()` UNIT type is returned by default**.

Function are also `type` in Rust (first class function behaviour(alike in python)).
type of add: `fn(u64, u64)->u64`.

IMPORTANT: **ALL PASSED PARAMETERS ARE ALSO `IMMUTABLE` BY DEFAULT** if `mut` is not specified explicitly.