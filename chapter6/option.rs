fn main() {
    let number1: Option<i32> = Some(100);
    let number2: Option<i32> = None;

    match number1 {
        Some(x) => println!("{}", x),
        None => println!("is none"),
    }

    match number2 {
        Some(x) => println!("{}", x),
        None => println!("is none"),
    }
}
