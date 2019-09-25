fn http_status() -> u16 {
    500
}
fn main() {
    let status = http_status();
    match status {
        // all the expressions
        500 => println!("Internal Server Error Status {}", status),
        200 => println!("Yay Got [{}]", status),
        _ => println!("ingnore value {}", status),
    }

    let x = 3;
    let y = -3;

    match (x, y) {
        (1, 1) => println!("one"),
        (2, j) => println!("two, {}", j),
        (_, 3) => println!("three"),
        (i, j) if i > 5 && j < 0 => println!("On guard!"),
        (_, _) => println!(":<"),
    }
}
