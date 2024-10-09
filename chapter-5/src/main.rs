#[derive(Debug)] // used to print struct values
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)] // used to print struct values
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // defining the area method
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {

    let mut user1 = User { // User instance
        active: true,
        username: String::from("user1name"),
        email: String::from("user1email"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("anotheremail");

    println!("{}", user1.email);

    let a = String::from("user2email");
    let b = String::from("user2name");

    let user2 = build_user(a, b);

    println!("{:?}", user2);

    let user3 = User {
        email: String::from("user3email"), // change only email
        ..user1 // rest filed like user 2
    };

    println!("{:?}", user3);


    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let scale = 3;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.", 
        rect1.area() // using the area method
    );

    println!(
        "The width of the rectangle is {}.", 
        rect1.width // using the width field or .width() method
    );

    println!("{:#?}", rect1);

    dbg!(&rect1);

}

fn build_user(email: String, username: String) -> User {
    
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}