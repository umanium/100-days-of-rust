fn main() {
    println!("Hello, world!");
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn add_nums(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let result = true;
        assert!(result);
    }

    #[test]
    fn add_two_test() {
        let result = add_two(10);
        assert_eq!(12, result);
    }

    #[test]
    fn add_nums_test() {
        let result = add_nums(2, 3);
        assert_eq!(5, result);
    }
}