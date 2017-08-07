### One of Rust's feature is Modules,

that allows us to create a function in a file, then use it in another file. This is a very useful feature, allows us to make a better organization in our programs.

So, as an example, let's make a "character", named "Bob", that will be controlled by us in the main program. The movement, though, is stored in another module named `movement.rs`. So, I will make our projects look like this.
```sh
$ tree

.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs
    └── movement.rs
```

Now let's add a movement to `go up` in the `movement.rs`. We want the `go_up` method accepting a `pair` of `(x,y)` and returning a `pair` too. The `y` will be added by one with this function. This will be the test of the `go_up`.
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_up() {
        let result_from_zeros = go_up((0, 0));
        let result_non_zeros_1 = go_up((5, 5));
        let result_non_zeros_2 = go_up((-5, 5));
        let result_non_zeros_3 = go_up((5, -5));
        assert_eq!((0, 1), result_from_zeros);
        assert_eq!((5, 6), result_non_zeros_1);
        assert_eq!((-5, 6), result_non_zeros_2);
        assert_eq!((5, -4), result_non_zeros_3);
    }
}
```

Now run it with `cargo test`. But wait! It looks like the tests are not executed. To make these executed, simply include the module `movement` in our `main.rs`.
```rust
mod movement;
```

It will complain that `go_up` is not found in the scope, it's because we haven't implement it yet. Let's implement it.
```rust
pub fn go_up(initial: (i32, i32)) -> (i32, i32) {
    println!("character go up");
    let x = initial.0;
    let y = initial.1;
    (x, y+1)
}
```

Now run `cargo test` again and the test should be `ok`.
```sh
$ cargo test
running 1 test
test movement::tests::test_go_up ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Now, let's make the main program to accept input that make `bob` go up if we inputted `w` key. Let's make it like the `guessing_game`, except we will call the `go_up` method if user inputted `w`.
```rust
mod movement;

use std::io;

fn main() {
    let mut bob = (0, 0);

    loop {
        println!("Bob is in position {:?}. Please input your movement (w, a, s, d), enter q to exit", bob);

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
    }
}
```

Now, we can repeat this process for another directions; Then we can look at `bob` "moving"!