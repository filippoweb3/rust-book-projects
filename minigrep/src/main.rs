use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1]; //the args 0 is the program's name!
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

}
