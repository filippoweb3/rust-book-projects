#[derive(Debug)]
struct Rectangle { // Rectangle Struct is a unique Type
    width: u32,
    height: u32,
}

impl Rectangle {

    fn area(&self) -> u32 { // area function into a method
        self.width * self.height
    }

    fn width(&self) -> bool { // method name can be the same as one of the fields
        self.width > 0
    }

}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    if rect.width() {
        println!{
            "The rectangle has a nonzero width of {}",
            rect.width
        }
    };

    println!(
        "The area of the rectangle is {} square pixels.",
         rect.area()
    );

    // println!("rect is {:#?}", rect)

    dbg!(&rect);

}

fn area(rectangle: &Rectangle) -> u32 { // area function
    rectangle.width * rectangle.height
}
