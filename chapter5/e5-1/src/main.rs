fn main() {
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    println!("{:?}", user1);
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{:?}", user2);

    let black = Color(0,0,0);
    println!("{:?}", black);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
