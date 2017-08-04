Hi again,

### In this 7th day of #100DaysOfCode, I want to tinker my guessing game further.

I want to add some conditions in the program to limit the game so and to answer the question like:
1. How if the user inputs the larger number for start than end?
2. How if user inputted too big number?
3. How user can quit from the game midway?

Let's tackle it one by one:
1. We want to inform user and resort to default numbers if start number is larger or equal than end. We can achieve this by making an `if` condition.
```rust
if start_num >= end_num {
    println!("Start number is greater than end number, assigining default numbers");
    start_num = 1;
    end_num = 100;
}
```
However, we should make the `start_num` and `end_num` mutable first using `mut` keyword.
```rust
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
```
This should be it.

2. Now, we can limit the numbers to be in range that we want to. Let's say the desired range for the numbers are `1-1000` for `start_num` and `100-10000` for `end_num`. We can add the conditionals after we do the previous check.

```rust
if start_num < 1 || start_num > 1000 {
    println!("Start number is out of range, assigning default number");
    start_num = 1;
}

if end_num < 100 || end_num > 10000 {
    println!("End number is out of range, assigning default number");
    end_num = 100;
}
```

3. Cool. Now, we want users to have a choice to quit mid-game. We probably want to assign the `q` key for it. Let's add other conditional inside the guessing loop.

```rust
if guess.trim() == "q" {
    println!("You choose to exit. Bye-bye!");
    return;
}
```

That's all for today! Have a nice day y'all!