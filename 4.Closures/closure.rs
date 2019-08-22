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
