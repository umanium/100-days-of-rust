mod point;
mod movement;

use point::Point;
use std::io;

fn main() {
    let mut bob = Point { x: 0, y: 0 };

    loop {
        println!("Bob is in position {}. Please input your movement (w, s, a, d), enter q to exit", bob);

        let mut movemt = String::new();
        io::stdin().read_line(&mut movemt)
            .expect("Failed to read line");

        if movemt.trim() == "q" {
            println!("You choose to exit. Bye-bye!");
            return;
        }
        else if movemt.trim() == "w" {
            println!("Now bob will go up");
            movement::go_up(&mut bob);
        }
        else if movemt.trim() == "s" {
            println!("Now bob will go down");
            movement::go_down(&mut bob);
        }
        else if movemt.trim() == "a" {
            println!("Now bob will go left");
            movement::go_left(&mut bob);
        }
        else if movemt.trim() == "d" {
            println!("Now bob will go right");
            movement::go_right(&mut bob);
        }
    }
}
