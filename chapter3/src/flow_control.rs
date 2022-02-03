pub fn if_expression() {
    let number = 3;
    if number % 4 == 0 {
        println!("{}", "mod 4")
    } else if number % 3 == 0 {
        println!("{}", "mod 3")
    } else {
        println!("{}", "unknown")
    }
}

pub fn let_with_if() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("{}", number);
}
