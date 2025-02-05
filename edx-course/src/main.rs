use std::fmt::Debug;
extern crate proc;
use proc::{test_macro};

#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Clone, Debug)]
struct Dog {
    name: String,
    age: u32,
}

trait Sound {
    fn make_sound(&self);
}

impl Sound for Person {
    fn make_sound(&self) {
        println!("Hello, my name is {:?}", self.name);
    }
}

impl Sound for Dog {
    fn make_sound(&self) {
        println!("Woof, my name is {}", self.name);
    }
}

fn multiply_any_number<T: std::ops::Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

trait Edible {
    fn eat(&self);
}

struct Bag<T: Edible + Clone + Debug> {
    content: T,
}
trait Shape: Debug {
    fn area(&self) -> f32;
}

#[derive(Debug)]
struct Circle {
    radius: u32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        (self.width * self.height) as f32
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius as f32 * self.radius as f32
    }
}

macro_rules! times_five {
    ($e:expr) => {
        $e * 5
    };
}

test_macro!();

fn main() {
    let valid_array = [1, 2, 3];

    // Technically could be None or Some
    // hint: look at the type of this variable
    let maybe_value: Option<i32> = safe_access(1, &valid_array);

    // However, let's match the function directly:
    match safe_access(1, &valid_array) {
        Some(value) => println!("We have a value: {value}"),
        None => println!("It doesn't exist :()")
    };

    if let Some(value) = safe_access(0, &valid_array) {
        println!("{value}"); // 1
    } else {
         println!("Nothing valid was found!");
    }

    // Custom traits

    let person = Person {
        name: "John".to_string(),
        age: 32,
    };

    let person2 = person.clone();

    let dog = Dog {
        name: "Rex".to_string(),
        age: 5,
    };

    person.make_sound();
    dog.make_sound();

    // Generics & Associated Types

    let multiply = multiply_any_number(10, 10);
    
    // Dynamic dispatch & Smart Pointers

    let circle = Circle { radius: 10 };
    let rectangle = Rectangle { width: 10, height: 10 };

    let circle_area = area_of_any_shape(Box::from(circle));
    let rect_area = area_of_any_shape(Box::from(rectangle));

    println!("Circle area: {}", circle_area);
    println!("Rectangle area: {}", rect_area);

    // let shapes: Vec<dyn Shape> = vec![circle, rectangle];
    // let shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle), Box::new(rectangle)];
    // println!("{:?}", shapes);

    // Macros

    let times_five = times_five!(10);

    println!("{}", times_five);

}

fn safe_access(index: usize, slice: &[i32]) -> Option<i32> {
    // We check to see if the length of the slice is zero, or
    // less than the requested index.  If it is, we return `None`
    if slice.len() == 0 || slice.len() < index {
        return None;
    }
    // Otherwise, we're good to return the requested item!
    Some(slice[index])
}

fn area_of_any_shape(shape: Box<dyn Shape>) -> f32 {
    shape.area()
}

fn square(x: u32) -> u32 {
    x * x
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn result_test() {
        let some_result: Result<(), ()> = Ok(());
        assert!(some_result.is_ok());
    }

    #[test]
    fn option_test() {
        let some_option: Option<u32> = Some(10);
        assert_eq!(some_option, Some(10));
    }

    #[test]
    fn does_square_work() {
        assert_eq!(square(2), 4);
    }

    #[test]
    #[should_panic(expected = "This is panic")]

    fn test_panic() {
        panic!("This is panic");
    }

}