use std::io;

fn main() {
    let mut x = 5; //mutable variable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SEC: u32 = 60 * 60 * 3; //constant, always immutable
    println!("Three hours have {THREE_HOURS_IN_SEC} seconds."); //all upper-case

    let y = 5;

    let y = y + 1; //shadowing

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); //shadowing within inner scope
    }

    println!("The value of y is: {y}");

    let _spaces = "   ";
    let _spaces = _spaces.len();

    // let mut spaces = "   ";
    // spaces = spaces.len(); --> we cannot mutate a variable's type

    // use the checked_add method to handle integer overflow
    let x = u32::max_value() - 2;
    let y = 3;

    match x.checked_add(y) {
        Some(v) => {
            println!("{} + {} = {}", x, y, v);
        }
        None => {
            println!("overflow!");
        }
    };

    // floating-point
    let x = 2.011;
    let y: f32 = 3.011;
    println!("{} and {}", x, y);

    // math operation
    let _truncated = -5 / 3;
    println!("{}", _truncated);

    // Boolean type
    let _t = true;

    let _hundred = '💯';
    println!("{}", _hundred);

    // Tuple
    let tup = (50, 25.5, 24.5, '💯');
    println!("{} + {} + {} = {}", tup.0, tup.1, tup.2, tup.3);

    let (a, b, c, d) = tup; //pattern matching to destructure tup
    println!("{} + {} + {} = {}", a, b, c, d);

    // Array
    let _a = [1, 2, 3, 4, 5];
    println!("{}", _a[0]);

    let _a = [3; 5]; // repetitions
    println!("{} {}", _a[0], _a[4]);

    indexer();
}

fn indexer() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is {element}")
}
