use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("/home/umanium/Documents/hello_doc.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read to string");
    println!("{}", contents)
    // assert_eq!(contents, "Hello, world!");
}
