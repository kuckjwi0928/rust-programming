pub fn basic(s: &String) -> usize {
    s.len()
}

pub fn variable_reference(s: &mut String) {
    s.push_str(", world");
    println!("{}", s);
}
