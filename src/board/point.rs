use std::ops::{Add, AddAssign, Sub};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub col: i32,
    pub row: i32,
}

impl Point {
    pub fn new(col: i32, row: i32) -> Self {
        Self { col: col, row: row }
    }

    pub fn new_null() -> Self {
        Self{
            col:-1,
            row:-1,
        }
    }

    pub fn new_null_action() -> [Self;2] {
        [Point::new_null(),Point::new_null()]
    }

    pub fn in_bounds(&self) -> bool {
        self.col >= 0 && self.col < 6 && self.row >= 0 && self.row < 6
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.col + other.col, self.row + other.row)
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.col - other.col, self.row - other.row)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.col += other.col;
        self.row += other.row;
    }
}
