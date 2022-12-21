use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;
use std::vec;
use std::{ops::{Add, Sub, Mul}, fmt, cmp};
use itertools::Itertools;
pub const NORTH: Vertex = Vertex { x: 0, y: 1, z: 0};
pub const SOUTH: Vertex = Vertex { x: 0, y: -1, z: 0};
pub const EAST:Vertex = Vertex { x: 1, y: 0, z: 0 };
pub const WEST:Vertex = Vertex { x: -1, y: 0, z: 0 };
pub const DOWN:Vertex = Vertex { x: 0, y: 0, z: -1};
pub const UP: Vertex = Vertex { x: 0, y: 0, z: 1 };

pub fn part_one(input: String) -> impl Display {
    let mut count = 0;
    let mut shapes = input.lines().map(|line| {
        count = count + 1;
        Shape::parse(line, count)
    }
    ).collect_vec();

    let len = shapes.len();
    for x in 0..len {
        let (left, right) = shapes.split_at_mut(x);

        if left.len() == 0 {
            continue;
        }
        for other in right {
            left.last_mut().unwrap().cull_matching_face(other);
        }
    }


    shapes.iter().map(|s| s.get_number_of_sides()).sum::<i32>()
}

pub fn part_two(input: String) -> impl Display {
    let mut count = 0;
    let mut shapes = input.lines().map(|line| {
        count = count + 1;
        Shape::parse(line, count)
    }
    ).collect_vec();

    let vertices = shapes.iter().map(|p| p.min_vertex).collect();

    let outside = BoundingBox::new(&vertices).outside_points(&vertices);
    vertices.iter().map(|p| p.sides_touching(&outside)).sum::<i32>()
}

fn parse_number(input: &str) -> i32 {
    i32::from_str(input).unwrap()
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Vertex {
    x: i32,
    y: i32,
    z: i32
}

impl Vertex {
    fn new(x: i32, y: i32, z: i32) -> Self {
        return Vertex { x, y, z };
    }

    fn neighbors(&self) -> HashSet<Vertex> {
        return HashSet::from([
            Vertex { x: self.x - 1, ..*self }, Vertex { x: self.x + 1, ..*self },
            Vertex { y: self.y - 1, ..*self }, Vertex { y: self.y + 1, ..*self },
            Vertex { z: self.z - 1, ..*self }, Vertex { z: self.z + 1, ..*self }
        ]);
    }
    fn sides_touching(&self, other: &HashSet<Vertex>) -> i32 {
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
#[derive(Clone)]
struct Shape {
    verticies: Vec<Vertex>,
    faces: Vec<Vec<Vertex>>,
    id: i32,
    min_vertex: Vertex
}

impl Shape {
    fn parse(line: &str, id: i32) -> Self {
        let split:Vec<&str> = line.trim().split(",").into_iter().collect();
        
        let min = Vertex::new(parse_number(split[0]), parse_number(split[1]), parse_number(split[2]));

        

        let mut verticies = Vec::new();
        for x in 0..=1 {
            for y in 0..=1 {
                for z in 0..=1 {
                    verticies.push(Vertex::new(min.x + x, min.y + y, min.z + z));
                }
            }
        }

        let mut faces = Vec::new();

        faces.push(vec![min, min + EAST, min + EAST + NORTH, min + NORTH]);
        faces.push(vec![min, min + EAST, min + EAST + UP, min + UP]);
        faces.push(vec![min, min + UP, min + NORTH + UP, min + NORTH]);
        faces.push(vec![min + UP, min + UP + NORTH, min + UP + NORTH + EAST, min + UP + EAST]);
        faces.push(vec![min + NORTH, min + NORTH + EAST, min + EAST + UP + NORTH, min + NORTH + UP]);
        faces.push(vec![min + EAST, min + EAST + NORTH, min + EAST + UP + NORTH, min + EAST + UP]);


        for face in faces.iter_mut() {
            face.sort_by_key(|v| (v.x, v.y, v.z));
        }

        return Shape { verticies: verticies, faces: faces, id, min_vertex: min }
    }
    fn get_number_of_sides(&self) -> i32 {
        return self.faces.len() as i32
    }

    fn cull_matching_face(&mut self, other: &mut Shape) -> Option<Vec<Vertex>> {

        let faces = self.faces.to_vec();
        for face_index in 0..faces.len() {
            let other_faces = other.faces.to_vec();
            for other_face_index in 0..other_faces.len() {
                if itertools::equal(faces[face_index].to_vec(), other_faces[other_face_index].to_vec()) {
                    self.faces.remove(face_index);
                    other.faces.remove(other_face_index);
                }
            }
        }
        Option::None
    }
}



#[derive(Debug)]
struct BoundingBox { maxx: i32, minx: i32, maxy: i32, miny: i32, maxz: i32, minz: i32 }

impl BoundingBox {
    fn new(points: &HashSet<Vertex>) -> BoundingBox {
        return BoundingBox {
            maxx: points.iter().fold(i32::MIN, |acc, a| max(acc, a.x)) + 1,
            minx: points.iter().fold(i32::MAX, |acc, a| min(acc, a.x)) - 1,
            maxy: points.iter().fold(i32::MIN, |acc, a| max(acc, a.y)) + 1,
            miny: points.iter().fold(i32::MAX, |acc, a| min(acc, a.y)) - 1,
            maxz: points.iter().fold(i32::MIN, |acc, a| max(acc, a.z)) + 1,
            minz: points.iter().fold(i32::MAX, |acc, a| min(acc, a.z)) - 1
        }
    }

    fn inside(&self, p: Vertex) -> bool {
        return p.x >= self.minx && p.x <= self.maxx &&
            p.y >= self.miny && p.y <= self.maxy &&
            p.z >= self.minz && p.z <= self.maxz;            
    }
    
    fn outside_points(&self, solid: &HashSet<Vertex>) -> HashSet<Vertex> {
        let mut res = HashSet::new();
        let mut work = Vec::from([Vertex { x: self.minx, y: self.miny, z: self.minz }]);
        while let Some(p) = work.pop() {
            if !solid.contains(&p) && !res.contains(&p) && self.inside(p) {
                res.insert(p);
                p.neighbors().iter().for_each(|n| work.push(*n));
            }            
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::Shape;


    
    #[test]
    fn shape_can_get_the_number_of_sides_based_on_verticies() {
        let shape = Shape::parse("1,1,1", 0);

        assert_eq!(shape.get_number_of_sides(), 6);
        
    }

    #[test]
    fn shape_can_cull_sides() {
        let mut shape_1 = Shape::parse("1,1,1", 0);
        let mut shape_2 = Shape::parse("2,1,1", 0);

        shape_1.cull_matching_face(&mut shape_2);

        assert_eq!(shape_1.get_number_of_sides(), 5);
        assert_eq!(shape_2.get_number_of_sides(), 5);
        
    }

    #[test]
    fn shape_can_cull_multiple_sides() {
        let mut shape_1 = Shape::parse("1,1,1", 0);
        let mut shape_2 = Shape::parse("2,1,1", 0);
        let mut shape_3 = Shape::parse("0,1,1", 0);
        let mut shape_4 = Shape::parse("1,2,1", 0);

        shape_1.cull_matching_face(&mut shape_2);
        shape_1.cull_matching_face(&mut shape_3);
        shape_1.cull_matching_face(&mut shape_4);

        assert_eq!(shape_1.get_number_of_sides(), 3);
        assert_eq!(shape_2.get_number_of_sides(), 5);
        assert_eq!(shape_3.get_number_of_sides(), 5);
        assert_eq!(shape_4.get_number_of_sides(), 5);
    }

    #[test]
    fn full_input_test() {

        let mut shape_1 = Shape::parse("1,1,1", 0);
        let mut shape_2 = Shape::parse("2,1,1", 0);
        let mut shape_3 = Shape::parse("0,1,1", 0);
        let mut shape_4 = Shape::parse("1,2,1", 0);

        let mut shapes = vec![shape_1, shape_2, shape_3, shape_4];

        let len = shapes.len();
        for x in 0..len {
            let (left, right) = shapes.split_at_mut(x);
    
            if left.len() == 0 {
                continue;
            }
            for other in right {
                left.last_mut().unwrap().cull_matching_face(other);
            }
        }

        assert_eq!(shapes[0].get_number_of_sides(), 3);
        assert_eq!(shapes[1].get_number_of_sides(), 5);
        assert_eq!(shapes[2].get_number_of_sides(), 5);
        assert_eq!(shapes[3].get_number_of_sides(), 5);
    }
}