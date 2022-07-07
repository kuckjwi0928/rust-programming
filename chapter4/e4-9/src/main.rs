fn main() {
    let my_string = String::from("hello world");
    let my_string_literal = "hello world";

    println!("{}", first_word(&my_string[..]));
    println!("{}", first_word(&my_string_literal[..]));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
