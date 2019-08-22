fn main() {
    let x_immutable = 10;
    println!("x value : {}", x_immutable);
    // x_immutable = 20;
    let mut y_mutable = 20;
    println!("y mut {}", y_mutable);
    y_mutable = 30;
    println!("y mut {}", y_mutable);
}
