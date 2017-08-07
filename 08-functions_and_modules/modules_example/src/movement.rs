pub fn go_up(initial: (i32, i32)) -> (i32, i32) {
    println!("character go up");
    let x = initial.0;
    let y = initial.1;
    (x, y+1)
}

pub fn go_down(initial: (i32, i32)) -> (i32, i32) {
    println!("character go down");
    let x = initial.0;
    let y = initial.1;
    (x, y-1)
}

pub fn go_left(initial: (i32, i32)) -> (i32, i32) {
    println!("character go left");
    let x = initial.0;
    let y = initial.1;
    (x-1, y)
}

pub fn go_right(initial: (i32, i32)) -> (i32, i32) {
    println!("character go right");
    let x = initial.0;
    let y = initial.1;
    (x+1, y)
}

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

    #[test]
    fn test_go_down() {
        let result_from_zeros = go_down((0, 0));
        let result_non_zeros_1 = go_down((5, 5));
        let result_non_zeros_2 = go_down((-5, 5));
        let result_non_zeros_3 = go_down((5, -5));
        assert_eq!((0, -1), result_from_zeros);
        assert_eq!((5, 4), result_non_zeros_1);
        assert_eq!((-5, 4), result_non_zeros_2);
        assert_eq!((5, -6), result_non_zeros_3);
    }

    #[test]
    fn test_go_left() {
        let result_from_zeros = go_left((0, 0));
        let result_non_zeros_1 = go_left((5, 5));
        let result_non_zeros_2 = go_left((-5, 5));
        let result_non_zeros_3 = go_left((5, -5));
        assert_eq!((-1, 0), result_from_zeros);
        assert_eq!((4, 5), result_non_zeros_1);
        assert_eq!((-6, 5), result_non_zeros_2);
        assert_eq!((4, -5), result_non_zeros_3);
    }

    #[test]
    fn test_go_right() {
        let result_from_zeros = go_right((0, 0));
        let result_non_zeros_1 = go_right((5, 5));
        let result_non_zeros_2 = go_right((-5, 5));
        let result_non_zeros_3 = go_right((5, -5));
        assert_eq!((1, 0), result_from_zeros);
        assert_eq!((6, 5), result_non_zeros_1);
        assert_eq!((-4, 5), result_non_zeros_2);
        assert_eq!((6, -5), result_non_zeros_3);
    }
}