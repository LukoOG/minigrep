use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = dbg!(args);

    let query = &args[1];
    let path = &args[2];
    let contents = fs::read_to_string(path).expect("Should have been able to read a file");

    println!("Searching for {query}");
    println!("contents: \n:{contents}");
}
