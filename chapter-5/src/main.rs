#[derive(Debug)] // used to print struct values

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}    

fn main() {

    let mut user1 = User { // User instance
        active: true,
        username: String::from("somename"),
        email: String::from("someemail"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail");

    println!("{}", user1.email);

    let a = String::from("user2email");
    let b = String::from("user2name");

    let user2 = build_user(a, b);

    println!("{:?}", user2);

    let user3 = User {
        email: String::from("user3email"), // change only email
        ..user2 // rest filed like user 2
    };

    println!("{:?}", user3);

}

fn build_user(email: String, username: String) -> User {
    
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}