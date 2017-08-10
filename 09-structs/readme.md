### Keeping tuple for every loop can be tiresome and unexpressive,

So we can create a data structure called `Point` to represent the position `bob` is in. To make a `struct`, we need to add a file `point.rs` to our project, and add it with a `struct` like this.
```rust
pub struct Point {
    pub x: i32,
    pub y: i32
}
```

And then we would have a method `translate` that will do a translation for the `Point`. First, we write a test for it.
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let mut point = Point { x: 0, y: 0 };

        point.translate(3, 5);
        let point_expected = Point { x: 3, y: 5 };
        assert!(point_expected == point);

        point.translate(5, -2);
        let point_expected_2 = Point { x: 8, y: 3 };
        assert!(point_expected_2 == point);
    }
}
```

Anyway, we need to implement `equal` too, because it is obvious that we need the `equal` for testing (and maybe for other purpose?). Let's first write some tests for the `equal` method.
```rust
#[test]
fn test_equal() {
    let point_1 = Point { x: 1, y: 5 };
    let point_2 = Point { x: 5, y: 5 };
    let point_3 = Point { x: 1, y: 5 };

    assert!(point_1 == point_3);
    assert!(point_1 != point_2);
}
```

For implementing `equal` method, we should implement `PartialEq` for our `struct`. It would be implemented like this.
```rust
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
```

After this, we can start implement our `translate` method. for implement all methods for the `struct`, we use `impl Point` and then define the function there.
```rust
impl Point {
    pub fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}
```

After this, we can implement our newly-created `Point` in the `movement` module. We want the `Point` to be `mut`able and change itself when it is moved. Let's modify the test for that.
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_up() {
        let mut point = Point { x: 0, y: 0 };
        let mut point_2 = Point { x: -5, y: -5 };

        go_up(&mut point);
        go_up(&mut point_2);

        let expected_point = Point { x: 0, y: 1 };
        let expected_point_2 = Point { x: -5, y: -4 };

        assert!(expected_point == point);
        assert!(expected_point_2 == point_2);
    }
}
```

We can change the implementation of the `go_up` method to pass the test. Don't forget to include the `struct` by using `use point::Point`.
```rust
use point::Point;

pub fn go_up(point: &mut Point) {
    println!("character go up");
    point.translate(0, 1);
}
```

And then repeat for all directions. After that, we can change the implementation in our `main.rs`.
```rust
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
```

### Anyway, we missed one functionality for our struct, it is the ability to include it into a string.

We can override the `fmt` method in `fmt::Display` for our `struct`.
```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "({}, {})", self.x, self.y)
    }
}
```