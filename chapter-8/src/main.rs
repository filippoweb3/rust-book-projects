use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new(); //creating empty vector

    let mut v = Vec::new();

    v.push(5); //pushing values into a vector
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5]; //create a vector with vec! macro

    let third = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no element."),
    }

    for i in &v { //iterating over vector values
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let data = "foo";
    let s = data.to_string();

    let mut s = String::from("foo");

    s.push_str("bar");

    println!("{s}");

    s.push('!');

    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 moved to s3

    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");

    for b in "hello".chars() {
        println!("{b}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The score for {} is {}", team_name, score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 25); // overwriting

    scores.entry(String::from("Yellow")).or_insert(200); //adding a key and value if not present
    scores.entry(String::from("Blue")).or_insert(100);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
