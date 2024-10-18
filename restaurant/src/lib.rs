mod front_of_house;

mod back_of_house;

use crate::front_of_house::hosting; // idiomatic "use"

use crate::front_of_house::hosting::add_to_waitlist; //unidiomatic use

pub fn eat_at_restaurant() {

    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist(); // idiomatic "use"

    add_to_waitlist(); //unidiomatic use

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

fn deliver_order() {}