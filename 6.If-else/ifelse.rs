fn main() {
    let is_ture = true;
    if is_ture {
        println!("isTure");
    } else {
        println!("isFalse");
    };

    let if_else_result = if is_ture {
        "Hello isTure"
    } else {
        "Hello isFalse"
    };
    println!("{}", if_else_result);
}
