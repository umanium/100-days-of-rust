use std::fmt;

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Point {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let point_1 = Point { x: 1, y: 5 };
        let point_2 = Point { x: 5, y: 5 };
        let point_3 = Point { x: 1, y: 5 };

        assert!(point_1 == point_3);
        assert!(point_1 != point_2);
    }

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