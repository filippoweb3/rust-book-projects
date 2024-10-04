fn main() {

{

    let s = "hello"; //string literal

    println!("{s}")

}

let mut s = String::from("hello"); //String type

s.push_str(", world!");

println!("{s}");

let x = 5;
let y = x;

println!("The value of x is {x}, the value of y is {y}");

{
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved into s2 (shallow copy)

    println!("{s2},  world!"); //cannot use s1 here
}

let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy

println!("s1 = {s1}, s2 = {s2}"); // can use both s1 and s2

let s = String::from("hello");

takes_ownership(s); // s is moved into fn, no longer valid after (drop function)

let x = 5;

makes_copy(x); // x is moved into fn, by x is Copy trait, and thus still valid afterwards

let s1 = gives_ownership();

println!("{s1}");

let s2 = String::from("hello");

let s3 = takes_and_gives_back(s2); // moves s2 into s3

println!("{s3}");


let s1 = String::from("hello");

let len = calculate_length(&s1); //borrows s1 to calculate length

println!("The length of '{s1}' is {len}."); //using s1 and len


let mut s = String::from("hello");

change(&mut s);

println!("{s}");

{
    let r1 = &mut s;
    println!("{r1}");
} // mutable reference goes out of scope

let r2 = &mut s;

println!("{r2}");

let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{r1} and {r2}");

let r3 = &mut s;
println!("{r3}");

let s = String::from("Hello, world!");

let hello = &s[0..6];
let world = &s[7..13];

println!("{hello} {world}");

let word = first_word(&s);

println!("{word}");

}







fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}