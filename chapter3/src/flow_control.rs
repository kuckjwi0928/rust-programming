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

pub fn loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", counter);
    println!("{}", result);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("{}", a[index]);
        index = index + 1;
    }

    for el in a.iter() {
        println!("{}", el);
    }

    for number in (1..4).rev() {
        println!("{}", number)
    }
}
