fn main() {
    let user1 = build_user(String::from("hello"), String::from("hello.com"));

    println!("{0}", user1.email);

    let user2 = User {
        email: String::from("ABC.com"),
        ..user1
    };

    // println!("{0}", user1.username); this does not work. username field moved from user1 to user2!

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {

    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }

}
