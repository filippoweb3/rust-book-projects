fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measure(5, "cm");

    let x = five();

    println!("The value of x is: {x}");

    if_state(4);

    countdown();

}

fn another_function(x: i32) {
    println!("The values of x is: {x}")
}

fn print_labeled_measure(x: i32, unit: &str) {
    println!("The measurement is: {x} {unit}")
}

fn five() -> i32 {

    5

}

fn if_state(x: i32){

    if x < 5 {

        println!("condition is true.")

    } else {

        println!("condition is false.")

    }

}

fn countdown(){

    for number in (1..4).rev(){

        println!("{number}!")

    }

    println!("Liftoff!!")

}
