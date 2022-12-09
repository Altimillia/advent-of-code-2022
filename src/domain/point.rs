use std::{ops::{Add, Sub, Mul}, fmt};

use num::clamp;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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