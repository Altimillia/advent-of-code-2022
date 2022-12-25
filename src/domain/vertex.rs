use std::{ops::{Add, Sub}, fmt, collections::HashSet };

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Vertex {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        return Vertex { x, y, z };
    }

    pub fn neighbors(&self) -> HashSet<Vertex> {
        return HashSet::from([
            Vertex { x: self.x - 1, ..*self }, Vertex { x: self.x + 1, ..*self },
            Vertex { y: self.y - 1, ..*self }, Vertex { y: self.y + 1, ..*self },
            Vertex { z: self.z - 1, ..*self }, Vertex { z: self.z + 1, ..*self }
        ]);
    }
    pub fn sides_touching(&self, other: &HashSet<Vertex>) -> i32 {
        return self.neighbors().iter().filter(|p| other.contains(&p)).count() as i32;
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}