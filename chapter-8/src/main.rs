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

}
