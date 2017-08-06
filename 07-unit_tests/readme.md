Sorry for the late again. It's so hard to be consistent everyday. But I will not go down. :D Let's continue with this thing!

### One of the most important thing in software development is Tests.

Now today we will take a peek how to do test in Rust. So, according to [Rust book on test organization](https://doc.rust-lang.org/book/second-edition/ch11-03-test-organization.html), unit tests should be included in the file that we want to test, as a module. Then, the test function should be marked with `#[test]`.

So, we can add a module in our `main.rs` to add the test that we want to do.
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn example_test() {
        let result = true;
        assert!(result);
    }
}
```

This test will always be passed because we just assert a `true` value. Now, let's run the test with `cargo test`.
```sh
$ cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 2.19 secs
     Running target/debug/deps/unit_tests-425e6ce9e595666b

running 1 test
test tests::example_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Now we have the test for `main.rs`! Now, we can test any `pub`lic or private functions in the file (The details about functions will be covered later). Let's do this! First, the `pub`lic function. Add the `add_two` function that add a number with 2.
```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}
```

And then write the test for it in the `tests` module.
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_test() {
        let result = add_two(10);
        assert_eq!(12, result);
    }
}
```

Note the `use super::*` we use before the test. This line is used to import functions from the parent module.

Now, the private function, let's add the `add_nums` function that add two integers.
```rust
fn add_nums(a: i32, b: i32) -> i32 {
    a + b
}
```

Because the test is just a module in a file, then we can add the function to the test freely, even the private one.
```rust
#[test]
fn add_nums_test() {
    let result = add_nums(2, 3);
    assert_eq!(5, result);
}
```

Now, let's run `cargo test` again.
```sh
$ cargo test

    Finished dev [unoptimized + debuginfo] target(s) in 2.37 secs
     Running target/debug/deps/unit_tests-425e6ce9e595666b

running 3 tests
test tests::add_nums_test ... ok
test tests::add_two_test ... ok
test tests::example_test ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

That's for unit testing! There are more, of course, but we will get to that later.