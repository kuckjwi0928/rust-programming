pub fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

pub fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

pub fn gives_ownership() -> String {
    String::from("test")
}

pub fn takes_and_gives_back(a_string: String) -> String {
    a_string
}