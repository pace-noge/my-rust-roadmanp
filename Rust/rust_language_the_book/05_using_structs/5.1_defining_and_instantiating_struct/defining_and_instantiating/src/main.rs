

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername"),
        active: true,
        sign_in_count: 1,
    };

    println!("email: {}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("email: {}", user1.email);

    let mut user2 = build_user(String::from("user2@example.com"), String::from("user2name"));

    println!("user2 email: {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username,
        email,       // shorthand
        username,   // shorthand
        active: true,
        sign_in_count: 1,
    }
}
