## Closures

> function without name but have information about the scope where decalared.

```rust
let simple_closure = || ();
```

`||` decalares what parameters we can pass. In simple closure we are taking no parameters.

`()` is unit type, that means no return. See 3.functions.

```rust
fn main() {
    let add_closure = |x, y| x + y;
    println!("{}", add_closure(3, 7));
    let even_odd_closure = |x: u64| {
        if x % 2 != 0 {
            return "odd";
        }
        return "even";
    };
    println!("{}", even_odd_closure(12));
    println!("{}", even_odd_closure(11));
}
```
