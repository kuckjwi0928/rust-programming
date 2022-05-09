pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

pub fn string_slice() {
    let s = String::from("hello world");
    println!("{}", &s[..5]);
    println!("{}", &s[6..]);
    println!("{}", &s[..]);
}
