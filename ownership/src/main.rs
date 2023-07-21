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
