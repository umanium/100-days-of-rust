use point::Point;

pub fn go_up(point: &mut Point) {
    println!("character go up");
    point.translate(0, 1);
}

pub fn go_down(point: &mut Point) {
    println!("character go down");
    point.translate(0, -1);
}

pub fn go_left(point: &mut Point) {
    println!("character go left");
    point.translate(-1, 0);
}

pub fn go_right(point: &mut Point) {
    println!("character go right");
    point.translate(1, 0);
}

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

    #[test]
    fn test_go_down() {
        let mut point = Point { x: 0, y: 0 };
        let mut point_2 = Point { x: -5, y: -5 };

        go_down(&mut point);
        go_down(&mut point_2);

        let expected_point = Point { x: 0, y: -1 };
        let expected_point_2 = Point { x: -5, y: -6 };

        assert!(expected_point == point);
        assert!(expected_point_2 == point_2);
    }

    #[test]
    fn test_go_left() {
        let mut point = Point { x: 0, y: 0 };
        let mut point_2 = Point { x: -5, y: -5 };

        go_left(&mut point);
        go_left(&mut point_2);

        let expected_point = Point { x: -1, y: 0 };
        let expected_point_2 = Point { x: -6, y: -5 };

        assert!(expected_point == point);
        assert!(expected_point_2 == point_2);
    }

    #[test]
    fn test_go_right() {
        let mut point = Point { x: 0, y: 0 };
        let mut point_2 = Point { x: -5, y: -5 };

        go_right(&mut point);
        go_right(&mut point_2);

        let expected_point = Point { x: 1, y: 0 };
        let expected_point_2 = Point { x: -4, y: -5 };

        assert!(expected_point == point);
        assert!(expected_point_2 == point_2);
    }
}