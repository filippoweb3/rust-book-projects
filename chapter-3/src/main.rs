fn main() {
    let mut x = 5; // making x mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 2; // specifying a constant

    println!("There are {:?} seconds in two hours.", TWO_HOURS_IN_SECONDS);

}

