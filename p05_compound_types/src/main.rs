fn main() {
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Access derectly
    let second = tup.1;
    println!("The second value is: {second}");

    // Tuple desturctue
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
