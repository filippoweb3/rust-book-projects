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

    //Custom traits

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
