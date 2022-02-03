pub fn basic() {
    let x = 2.0;
    let y: f32 = 3.0;

    println!("{}", x);
    println!("{}", y);

    let mut t = true;
    t = false;
    println!("{}", t);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    // array
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    // fill
    let a = [3; 5];
    println!("{}", a[0]);
}
