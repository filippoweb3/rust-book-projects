fn main() {
    let mut x = 5; // making x mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 2; // specifying a constant

    println!("There are {:?} seconds in two hours.", TWO_HOURS_IN_SECONDS);

    let x = 5;
    let x = " "; // shadowing

    let x = 1_000;
    println!("The value of x is: {x}");

    let x = 2.1;

    let x = true;

    let x = 'Z';

    let tup = (100, 1.1, 'Z');
    let (x, y, z) = tup;
    println!("The first element in the tuple is: {:?}.", tup.0);

    let a = [1, 2, 3];
    let first = a[0];

    another_function(5, "kg");
    let x = another_function(5, "kg");

    let condition = true;
    let number = if condition {1} else {2};

    println!("The value of the number is: {}", number);

    if number % 2 == 0 && number % 1 == 0 {
        println!("The number is divisible by 1 and 2.");
    } else if number % 2 == 0 || number % 1 == 0 {
        println!("The number is divisible by 1 or 2.");
    } else {
        println!("The number is not divisible by 1 or 2.");
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;

    }

    println!("End count = {count}");


    let mut number = 3;

    loop {

        println!("{number}");

        number -= 1;

        if number == 0 {
            break
        }

    }

    println!("GO!!!");


    let mut number = 3;

    while number != 0 {

        println!("{number}");

        number -= 1;

    }

    println!("GO!!!");

    for number in (1..4).rev() {

        println!("{number}");

    }

    println!("GO!!!");

}

fn another_function(x: i32, unit: &str) -> i32 { // specifying the return type

    let y = x * 2; // doing something with x

    println!("The weight is {}{}.", y, unit); // printing x and y

    y

}

