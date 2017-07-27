### After we can read from user inputs,

I'm curious about reading files. So in this day #100DaysOfCode, I want to do the follows:
- Read a text file line-by-line
- Store the lines to an array
- Count the array elements
- Profit!!!

Let's first create a project using `cargo`:
```
cargo new reading_lines --bin
```

Now, we have to prepare a text file to read. Let's say the file named `text_input.txt`. Store it in `~/Documents` directory, so we won't have to hassle with copying file when building.

Now let's open the `main.rs` and start coding!

### Opening the file
Let's start from a simple file reading. We want to open a file, then dump the content into a string variable.
```rust
use std::fs::File;
use std::io::prelude::*;

let mut file = File::open("/home/umanium/Documents/hello_doc.txt").expect("Unable to open file");
let mut contents = String::new();
file.read_to_string(&mut contents).expect("Unable to read to string");
println!("{}", contents)
```

Pretty straightforward.

### Reading with BufRead
To read a file line-by-line efficiently, we can read using an internal buffer, called `BufRead` in Rust.

A locked standard input `stdin` implements BufRead, so we can use an `io::stdin` to read by line:
```rust
use std::io;
use std::io::prelude::*;

let stdin = io::stdin();
for line in stdin.lock().lines() {
    println!("Hello {}", line.unwrap())
}
```

Now, to read a **file** line-by-line, we can open the file and convert it into a `BufReader` that implements `BufRead`.
```rust
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

let file = File::open("/home/umanium/Documents/hello_doc.txt").expect("Unable to open file");
let f = BufReader::new(file);
for line in f.lines() {
    println!("ugh {}", line.unwrap());
}
```

To get the number of lines in the text file, we can use the method `count()` in `f.lines()`
```rust
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

let file = File::open("/home/umanium/Documents/hello_doc.txt").expect("Unable to open file");
let f = BufReader::new(file);
println!("{}", f.lines().count());
```