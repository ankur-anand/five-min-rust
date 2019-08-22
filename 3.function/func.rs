fn add(x: u64, y: u64) -> u64 {
    return x + y;
}

fn main() {
    let x: u64 = 20;
    let y: u64 = 30;
    let result = add(x, y);
    println!("result: {}", result);
}
