extern crate rand;
extern crate getopts;

use std::io;
use std::cmp::Ordering;
use std::env;
use std::string::String;
use rand::Rng;
use getopts::Options;

fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("s", "start", "define a start range", "NUM");
    opts.optopt("e", "end", "define an end range", "NUM");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        println!("{}", opts.usage(&brief));
        return;
    }

    let start_str = match matches.opt_str("s") {
        Some(s) => { s },
        None => { String::from("1") }
    };

    let end_str = match matches.opt_str("e") {
        Some(e) => { e },
        None => { String::from("100") }
    };

    let mut start_num: u32 = match start_str.parse() {
        Ok(num) => { num },
        Err(_) => { 
            println!("Start is not a valid number, assigning default number");
            1
        }
    };

    let mut end_num: u32 = match end_str.parse() {
        Ok(num) => { num },
        Err(_) => { 
            println!("End is not a valid number, assigning default number");
            100
        }
    };

    if start_num >= end_num {
        println!("Start number is greater than end number, assigining default numbers");
        start_num = 1;
        end_num = 100;
    }

    if start_num < 1 || start_num > 1000 {
        println!("Start number is out of range, assigning default number");
        start_num = 1;
    }

    if end_num < 100 || end_num > 10000 {
        println!("End number is out of range, assigning default number");
        end_num = 100;
    }

    println!("=== Guess the number ===");
    let secret_number = rand::thread_rng().gen_range(start_num, end_num+1);

    println!("I have a secret number from {} to {}, guess it!", start_num, end_num);

    loop {
        println!("Please input your guess, enter q to exit");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "q" {
            println!("You choose to exit. Bye-bye!");
            return;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
