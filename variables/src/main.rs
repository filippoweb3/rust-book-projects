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

}
