mod memory_safe;
mod function_ownership;
mod reference;

fn main() {
    memory_safe::basic();

    let str = String::from("kuckjwi");
    let x = 5;

    function_ownership::takes_ownership(str);
    function_ownership::makes_copy(x);

    // error (because move str to takes_ownership)
    // println!("{}", str);
    println!("{}", x);

    let str2 = function_ownership::gives_ownership();
    let str3 = String::from("hello");
    let str4 = function_ownership::takes_and_gives_back(str3);
    println!("{}", str2);
    // error (because move str3 to str4)
    // println!("{}", str3);
    println!("{}", str4);

    let str = String::from("kuckjwi");
    let len = reference::basic(&str);

    println!("{} {}", str, len);

    let mut hello = String::from("Hello");

    reference::variable_reference(&mut hello);
}
