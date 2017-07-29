use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("There is {} arguments in the command", args.len()-1);
    println!("Your command is {}", args[0]);
    if args.len() > 1 {
        println!("The first argument is {}", args[1])
    }
}
