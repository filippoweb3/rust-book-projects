fn main() {

    {

        let s = "hello"; //string literal

        let mut s1 = String::from("hello"); //String type

        s1.push_str(", world!");

        let s2 = s1.clone(); //clone String type to the heap

        println!("{s1} {s2}");



        // Ownership and Functions

        takes_ownership(s1);

        let x = 5;

        makes_copy(x);


        // Transfer Ownership

        let s1 = gives_ownership(); //gives ownership to s1

        let s2 = String::from("hello");
        
        let s3 = takes_and_gives_back(s2); //takes ownership of s2 and gives it to s3

        println!("{s1}");

        // this won't work --> println!("{s2}")

        // Problem - if we move X into a fn, to use X afterwards the fn must return X

        // workaround
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of {s2} is {len}.");

        // Elegant solution - provide a reference to the String value

        let s1 = String::from("hello");

        let len = calculate_length_ref(&s1);

        println!("The length of {s1} is {len}.");   

        // Mutable References

        let mut s = String::from("hello");
        
        change(&mut s);
        
        println!("{s}"); // this prints "hello, world!", so we mutated s using a reference!

        // the pointer has access to the data that can be mutated
        // in the previous example we calculated a length, no mutation
        // here we mutated the data!

        // No multiple mutable references!

        {
            let r1 = &mut s;
        } // we must take r1 out of scope

        let r2 = &mut s; // and set another mutable reference

        println!("{r2}");

        // No immutable and mutable references!
        // if you create an immutable reference you do not expect changes to what you are referencing to

        let r3 = &s;
        let r4 = &s;

        println!("{r3} {r4}"); // r3 and r4 will not be used after this point

        let r5 = &mut s; // we can have a mutable reference, scope for r3 and r4 ends here

        // println!("{r5}");

        // let ref_to_nothing = dangle(); this won't work

        let no_ref_to_nothing = no_dangle();

        let word = first_word(r5);

        println!("{word}"); // this will print value 6

        s.clear(); // empties the string

        println!("{s}"); // s now has value ""

        // the word index is still valid but the word is now empty

        // Slices 

        let s = String::from("hello world");

        let hello = &s[0..5]; // [start_index..end_index], end_index is one more than the last position of the slice
        let world = &s[6..11];

        println!("{hello} {world}");

        let slice = &s[0..2]; // is the same as &s[..2]

        let len = s.len();

        let slice = &s[3..len]; // same as &s[3..]
        let slice = &s[0..len]; // same as &s[..]

    }
    
}

fn takes_ownership(some_string: String) {
    println!("{some_string}")
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}")
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // also returns s to give back ownership
}

fn calculate_length_ref(s: &String) -> (usize) { // s is a ref to a String
    s.len()
} // s goes out of scope, but does not have ownership over what it refers to and this String is not dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn dangle() -> &String { 
//    let s = String::from("hello");
//    &s
//} // s goes out of scope and is dropped, the ref will point to empty memory!

fn no_dangle() -> String {

    let s = String::from("hello");

    s

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b' ' is equal to 32
            println!("{item}");
            return i;
        }
    }

    s.len()
}