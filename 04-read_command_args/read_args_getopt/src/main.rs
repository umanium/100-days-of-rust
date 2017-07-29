extern crate getopts;

use getopts::Options;
use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("n", "num", "add a number", "NUM");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        println!("{}", opts.usage(&brief));
        return;
    }

    let num_str = match matches.opt_str("n") {
        Some(n) => { n },
        None => { panic!("No number!") }
    };
    let num: u32 = match num_str.parse() {
        Ok(num) => { num },
        Err(_) => { panic!("Not a number!") }
    };
    println!("You inputted {}", num)
}
