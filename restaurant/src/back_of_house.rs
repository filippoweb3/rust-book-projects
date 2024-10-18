fn fix_incorrect_order() {
    cook_order();
    super::deliver_order(); //super syntax to go out of back_of_house
}

fn cook_order() {}

pub struct Breakfast { //public struct
    pub toast: String, //we need to specify public fields
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub enum Appetizer { //public enum
    Soup, //also public variants
    Salad,
}