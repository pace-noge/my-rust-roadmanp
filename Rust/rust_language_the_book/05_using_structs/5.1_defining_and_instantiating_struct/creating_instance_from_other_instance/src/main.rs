
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: i64,
}


fn main() {
    let user1 = User {
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1
        
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("User1 email {}, user2 email {}", user1.email, user2.email);

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };

    println!("user1 email: {}, user3 email {}, user3 is active {}", user1.email, user3.email, user3.active);
}
