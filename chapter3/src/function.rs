pub fn another_function(x: i32, y: i32) {
    println!("x value is {}", x);
    println!("y value is {}", y);
}

pub fn function_block() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("{}", y);
}

pub fn five() -> i32 {
 5
}
