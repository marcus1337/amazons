use std::ops::{Add, AddAssign, Sub};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

    pub fn get_from(points: [Point;2]) -> Point {
        points[0]
    }
    
    pub fn get_to(points: [Point;2]) -> Point {
        points[1]
    }

    pub fn is_move(points: [Point;2]) -> bool {
        return points[0] != points[1]
    }

    pub fn is_drop(points: [Point;2]) -> bool {
        return points[0] == points[1]
    }

    pub fn in_bounds(&self, grid_size : i32) -> bool {
        self.col >= 0 && self.row >= 0 && self.col < grid_size && self.row < grid_size
    }

    pub fn step_towards(&mut self, other: Point) -> bool {
        if self.col < other.col {
            self.col += 1;
        } else if self.col > other.col {
            self.col -= 1;
        }
        if self.row > other.row {
            self.row -= 1;
        } else if self.row < other.row {
            self.row += 1;
        }
        *self != other
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
