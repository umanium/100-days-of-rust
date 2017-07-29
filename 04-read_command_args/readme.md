### Welcome to day 5!
It's 1/20 way already! Consistency is hard, but we're getting there!

Okay so today I want to make my program read arguments from command line, so it can be run like `rm -rf /` or any command line program. So we have to create a code that read arguments from command line. Let's start with the simple one:
```rust
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("There is {} arguments in the command", args.len()-1);
    println!("Your command is {}", args[0]);
    if args.len() > 1 {
        println!("The first argument is {}", args[1])
    }
}
```

`env::args().collect()` will return a `vector` that can be consumed later in the program. Our custom arguments started with the index `[1]` because the index `[0]` is our program command itself.

Now, we can run `cargo run` to run our program. However to add arguments in the program run with cargo, we have to add `--` before our arguments. Let's do with an example:
```sh
> cargo run -- example
There is 1 arguments in the command
Your command is target/debug/read_args
The first argument is example
```

We did it! Now, let's do a more complicated arguments reading with `getopts` library. First, we add `getopts` to `[dependencies]` in `Cargo.toml`:
```toml
[dependencies]
getopts = "0.2"
```

Now let's add an optional flag `-h` to our program, and adding help for our options.
```rust
extern crate getopts;

use getopts::Options;
use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    
    opts.optflag("h", "", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        println!("{}", opts.usage(&brief));
        return;
    }
}
```
First, we get the args using the previous method `env::args().collect()`. And then we have to devine our options using `Options::new()`. The flag we want to add, `-h`, we can add using the `opts.optflag()` method. This way, it is only considered as a flag and can be checked by `opt_present()` method. But, before we checked it, we have to do matching with `match opts.parse()` method.

Now I will run it using `cargo run -- -h`
```sh
> cargo run -- -h
Hello, world!
Usage: target/debug/read_args_getopt [options]

Options:
    -h                  print this help menu
```

Now let's add an option to bring a number to our program. Let's say it will be called with `-n` option.
```rust
opts.optopt("n", "", "add a number", "NUM")
```

Now, we add a method to get the argument from our `-n` option and check if it is a number or not.
```rust
let num_str = match matches.opt_str("n") {
    Some(n) => { n },
    None => { panic!("No number!") }
};
let num: u32 = match num_str.parse() {
    Ok(num) => { num },
    Err(_) => { panic!("Not a number!") }
};
println!("You inputted {}", num)
```

Anyway, we can use a long name for our args, by adding the second arguments in the `optopt()` and `optflag()` method.
```rust
opts.optflag("h", "help", "print this help menu");
opts.optopt("n", "num", "add a number", "NUM");
```

And that's it for today! Have a nice day!