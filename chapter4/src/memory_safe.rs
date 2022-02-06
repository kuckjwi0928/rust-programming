pub fn basic() {
    let s1 = String::from("hello");
    // move s1 to s2
    let s2 = s1;

    // error (because move s1 to s2)
    // println!("{}", s1);
    println!("{}", s2);

    // clone
    let s3 = s2.clone();
    println!("{}", s3);

    // integer clone
    let x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);
}
