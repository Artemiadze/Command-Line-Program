use std::env;
use std::fs;  // For working with files

fn main() {
    // Collecting the command line arguments into a vector
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)    //read file_path and open file
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}