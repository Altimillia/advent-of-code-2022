use std::{ops::{Add, Sub, Mul}, fmt, cmp};

use num::clamp;

pub const NORTH: Point = Point { x: 0, y: 1};
pub const SOUTH: Point = Point { x: 0, y: -1};
pub const EAST:Point = Point { x: 1, y: 0 };
pub const WEST:Point = Point { x: -1, y: 0 };

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        return Point { x ,y }
    }

    pub fn scale(self, rhs: i32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }

    pub fn normalize(self) -> Self {
        Self { x: clamp(self.x, -1, 1), y: clamp(self.y, -1, 1)}
    }

    pub fn parse(input: String) -> Self {
        let split:Vec<&str> = input.trim().split(",").into_iter().collect();
        Self { x: split[0].parse::<i32>().unwrap(), y: split[1].parse::<i32>().unwrap() }
    }
    pub fn all_points_between(&self, rhs: Point) -> Vec<Point> {
        let x_min = cmp::min(self.x, rhs.x);
        let y_min = cmp::min(self.y, rhs.y);
        let x_max = cmp::max(self.x, rhs.x);
        let y_max = cmp::max(self.y, rhs.y);
        let mut points:Vec<Point> = Vec::new();

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                points.push(Point::new(x, y));
            }
        }

        return points;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}