### I missed a day but it doesn't matter,
And now I want to do the *guessing game* tutorial in [The Rust Programming Language book](https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html). So in this game, the program will have a number and the user have to guess it. And here are the steps of program:

First, we will have to accept input from user, because user will enter number to guess the computer's number. We will use the good old `io::stdin()` to do that.
```rust
use std::io;

println!("Please input your guess");

let mut guess = String::new();
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

Now, because this is about the user guessing a number from the computer, the input should be casted to integer. We can use the following code to do that.
```rust
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

After we have the guess, we need to assign a number and comparing them. we can use the `std::cmp::Ordering` to do that.
```rust
use std::cmp::Ordering;

let secret_number = 28;
match guess.cmp(&secret_number) {
    Ordering::Less    => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal   => println!("You win!")
}
```

Now, we don't want the number be a constant, right? We can use the `rand` library to do a random number generation. However, this is an external library, so we should include it in our `Cargo.toml`:
```toml
[dependencies]

rand = "^0.3.14"
```

And then we can `import` it in the program by using `extern crate`. After that, we should have the method to generate a random number from 1 to 100.
```rust
extern crate rand;
use rand::Rng;

let secret_number = rand::thread_rng().gen_range(1, 101);

println!("The secret number is: {}", secret_number);
```

Now, the program is as follows. We put the guessing part in a `loop` so the program will prompt user to do guessing until they are right.
```rust
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("=== Guess the number ===");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

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
```

This is it for now, we should get the guessing game running after this. See you tomorrow!