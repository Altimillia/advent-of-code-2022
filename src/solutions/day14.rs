use std::{fmt::Display, collections::HashMap, cell::Cell, borrow::BorrowMut};

use itertools::Itertools;

use crate::domain::point::Point;

pub fn part_one(input: String) -> impl Display {
    let rock_structures = input
        .lines()
        .map(|l| RockStructure::new(l))
        .collect_vec();

    let grid = Grid::new(rock_structures, Point { x: 500, y: 0 });
    grid.grid_points.len()
}

pub fn part_two(input: String) -> impl Display  {
    0
}

struct RockStructure {
    positions: Vec<Point>
}

impl RockStructure {
    fn new(line: &str) -> Self {
        let points = line.split("->")
            .into_iter()
            .map(|coord| Point::parse(coord.to_string()))
            .collect_vec();

        RockStructure { positions: points }
    }
}

struct Grid {
    grid_points: HashMap<Point, GridPoint>,
    sand_origination: Point,
    min: Point,
    max: Point
}

impl Grid {
    fn new(rock_structures: Vec<RockStructure>, sand_origination: Point) -> Self {

        let mut grid_points = HashMap::new();
        let min_x = rock_structures
            .iter()
            .map(|p| &p.positions)
            .flatten()
            .map(|p| p.x)
            .min()
            .unwrap();

        let max_x = rock_structures
            .iter()
            .map(|p| &p.positions)
            .flatten()
            .map(|p| p.x)
            .max()
            .unwrap();

        let min_y = rock_structures
            .iter()
            .map(|p| &p.positions)
            .flatten()
            .map(|p| p.y)
            .min()
            .unwrap();

        let max_y = rock_structures
            .iter()
            .map(|p| &p.positions)
            .flatten()
            .map(|p| p.y)
            .max()
            .unwrap();



        for x in min_x..max_x {
            for y in min_y..max_y {
                println!("{}-{}", x,y);
                grid_points.insert(Point::new(x,y), GridPoint { occupied: Cell::new(GridObject::Air) });
            }
        }

        rock_structures.iter().for_each(|rock| {
            rock.positions.iter().for_each(|pos| { 
                //println!("{}", pos);
                grid_points.get(pos).unwrap().update_point(GridObject::Rock);
            });
        });

        Grid { grid_points: grid_points, min: Point::new(min_x, min_y), max: Point { x: max_x, y: max_y }, sand_origination }
    }
}


enum GridObject {
    Sand,
    Rock,
    Air
}

struct GridPoint {
    occupied: Cell<GridObject>
}

impl GridPoint {
    fn update_point(&self, grid_object: GridObject) {
        self.occupied.set(grid_object);
    }
}