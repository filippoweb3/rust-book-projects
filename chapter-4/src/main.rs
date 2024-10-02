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

}
