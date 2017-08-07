mod movement;

use std::io;

fn main() {
    let mut bob = (0, 0);

    loop {
        println!("Bob is in position {:?}. Please input your movement (w, s, a, d), enter q to exit", bob);

        let mut movemt = String::new();
        io::stdin().read_line(&mut movemt)
            .expect("Failed to read line");

        if movemt.trim() == "q" {
            println!("You choose to exit. Bye-bye!");
            return;
        }
        else if movemt.trim() == "w" {
            println!("Now bob will go up");
            bob = movement::go_up(bob);
        }
        else if movemt.trim() == "s" {
            println!("Now bob will go down");
            bob = movement::go_down(bob);
        }
        else if movemt.trim() == "a" {
            println!("Now bob will go left");
            bob = movement::go_left(bob);
        }
        else if movemt.trim() == "d" {
            println!("Now bob will go right");
            bob = movement::go_right(bob);
        }
    }
}
