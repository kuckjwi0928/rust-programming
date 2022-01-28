pub fn basic() {
    // mutable
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);
}

pub fn shadowed() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("x: {}", x);
}
