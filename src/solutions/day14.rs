use core::time;
use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    fmt::Display,
    io::{self, stdin, stdout, Write},
    rc::Rc,
    thread::{self, Builder},
};

use itertools::Itertools;

use crate::domain::point::{Point, SOUTH};

pub fn part_one(input: String) -> impl Display {

    0
}

pub fn part_two(input: String) -> impl Display {
    let mut grid_objects = input
        .lines()
        .map(|l| ParsedGridObject::new(l))
        .collect_vec();

    grid_objects.push(ParsedGridObject {
        positions: vec![Point { x: 500, y: 0 }],
        grid_object: GridObject::Air,
    });

    let mut grid = Grid::new(grid_objects, Point { x: 500, y: 0 }, true);

    let map = grid.print_grid(PrintOptions::Objects);
    print!("{}", map);

    let mut count = 0;
    let mut cursor:Option<Point> = Option::None;
    cursor = grid.tick(cursor);
    while count < 10000000 && cursor.is_some() {
        count = count + 1;
        cursor = grid.tick(cursor);
    }

    println!("");
    println!("{}", grid.print_grid(PrintOptions::Objects));
    println!("");

    let sand_count = grid
        .grid_points
        .values()
        .filter(|f| matches!(f.occupied, GridObject::Sand))
        .collect_vec()
        .len();
    println!("");
    println!("Iterations {}", count);
    println!("Units of Sand {}", sand_count);

    0
}



struct ParsedGridObject {
    positions: Vec<Point>,
    grid_object: GridObject,
}

impl ParsedGridObject {
    fn new(line: &str) -> Self {
        let points: HashSet<Point> = line
            .split("->")
            .into_iter()
            .map(|coord| Point::parse(coord.to_string()))
            .collect_vec()
            .windows(2)
            .map(|p| p[0].all_points_between(p[1]))
            .flatten()
            .collect();

        ParsedGridObject {
            positions: points.into_iter().collect_vec(),
            grid_object: GridObject::Rock,
        }
    }
}

struct Grid {
    grid_points: HashMap<Point, GridPoint>,
    sand_origination: Point,
    min: Point,
    max: Point,
}

impl Grid {
    fn new(grid_objects: Vec<ParsedGridObject>, sand_origination: Point, add_floor: bool) -> Self {
        let mut grid_points = HashMap::new();
        let min_x = grid_objects
            .iter()
            .map(|p| &p.positions)
            .flatten()
            .map(|p| p.x)
            .min()
            .unwrap()
            - 150;

        let max_x = grid_objects
            .iter()
            .map(|p| &p.positions)
            .flatten()
            .map(|p| p.x)
            .max()
            .unwrap()
            + 150;

        let min_y = grid_objects
            .iter()
            .map(|p| &p.positions)
            .flatten()
            .map(|p| p.y)
            .min()
            .unwrap();

        let max_y = grid_objects
            .iter()
            .map(|p| &p.positions)
            .flatten()
            .map(|p| p.y)
            .max()
            .unwrap()
            + 2;

        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                grid_points.insert(
                    Point::new(x, y),
                    GridPoint {
                        occupied: GridObject::Air,
                        at_rest: true,
                    },
                );
            }
        }

        grid_objects.iter().for_each(|object| {
            object.positions.iter().for_each(|pos| {
                grid_points
                    .get_mut(pos)
                    .unwrap()
                    .update_point(object.grid_object, true);
            });
        });

        if add_floor {
            for x in min_x..max_x + 1 {
                grid_points
                    .get_mut(&Point { x: x, y: max_y })
                    .unwrap()
                    .update_point(GridObject::Rock, true);
            }
        }


        Grid {
            grid_points: grid_points,
            min: Point::new(min_x, min_y),
            max: Point { x: max_x, y: max_y },
            sand_origination: sand_origination,
        }
    }

    fn tick(&mut self, sand: Option<Point>) -> Option<Point> {
        let mut sand_moved = false;
        let mut sand_point: Option<Point> = Option::None;
        if sand.is_none() {
            let point = &mut self
                .grid_points
                .iter()
                .find(|(p, gp)| {
                    matches!(gp.occupied, GridObject::Sand) && gp.at_rest == false
                })
                .map(|p| p.0.clone());
            sand_point = point.clone();
        }
        else {
            sand_point = sand;
        }

        if sand_point.is_some() {
            let p = sand_point.unwrap();
            let possible_paths = vec![
                p + Point::new(0, 1),
                p + Point::new(-1, 1),
                p + Point::new(1, 1),
            ];
            let point = self.get_path_for_sand(possible_paths.to_vec());
            if point.is_some() {
                let next_position = point.unwrap();

                if self.grid_points.contains_key(&next_position) {
                    self.grid_points
                        .entry(next_position)
                        .and_modify(|gp| gp.update_point(GridObject::Sand, false));
                } else {
                    if !self.point_in_bounds(next_position) {
                        self.add_column(next_position.x);
                        self.grid_points
                            .entry(next_position)
                            .and_modify(|gp| gp.update_point(GridObject::Sand, false));
                    }
                }

                self.grid_points
                    .entry(p)
                    .and_modify(|gp| gp.update_point(GridObject::Air, true));
                
                sand_moved = true;
                return Option::Some(next_position);
            } else {
                self.grid_points
                    .entry(p)
                    .and_modify(|gp| gp.update_point(GridObject::Sand, true));
            }
        }
        if !sand_moved
            && matches!(
                self.grid_points
                    .get(&self.sand_origination)
                    .unwrap()
                    .occupied,
                GridObject::Sand
            )
        {
            return Option::None;
        } 
        if !sand_moved {
            let sand_spawn = self.sand_origination;
            self.grid_points.entry(sand_spawn).and_modify(|gp| gp.update_point(GridObject::Sand, false));
            return Option::Some(sand_spawn);
        }

        return Option::None;
    }

    fn add_column(&mut self, x: i32) {
        println!("Adding a column {}", x);
        for y in self.min.y..=self.max.y {
            self.grid_points.insert(
                Point::new(x, y),
                GridPoint {
                    occupied: GridObject::Air,
                    at_rest: true,
                },
            );
        }

        self.grid_points
            .entry(Point::new(x, self.max.y)).and_modify(|gp| gp.update_point(GridObject::Rock, true));
        let min_x = self.grid_points.iter().map(|p| &p.0.x).max().unwrap();

        let max_x = self.grid_points.iter().map(|p| &p.0.x).min().unwrap();
        self.max = Point::new(*max_x, self.max.y);
        self.min = Point::new(*min_x, self.min.y);
    }
    fn point_in_bounds(&self, point: Point) -> bool {
        if point.x < self.min.x
            || point.x > self.max.x
            || point.y < self.min.y
            || point.y > self.max.y
        {
            return false;
        }

        true
    }

    fn get_path_for_sand(&self, possible_paths: Vec<Point>) -> Option<Point> {
        let mut path: Option<Point> = Option::None;
        for x in possible_paths {
            let point = match self.grid_points.get(&x) {
                Some(grid_point) => match grid_point.occupied {
                    GridObject::Air => Option::Some(x),
                    _ => Option::None,
                },
                None => Option::Some(x),
            };
            if point.is_some() {
                path = Option::Some(point.unwrap());
                break;
            }
        }
        return path;
    }

    fn print_grid(&self, print_options: PrintOptions) -> String {
        let mut map = String::new();
        for y in self.min.y..=self.max.y {
            map.push_str("\n");
            for x in self.min.x..=self.max.x {
                let gp = self.grid_points.get(&Point::new(x, y)).unwrap();
                match gp.occupied {
                        GridObject::Sand => map.push_str("o"),
                        GridObject::Rock => map.push_str("#"),
                        GridObject::Air => map.push_str("."),
                }
            }
        }

        return map;
    }
}

enum PrintOptions {
    Objects,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum GridObject {
    Sand,
    Rock,
    Air,
}

struct GridPoint {
    occupied: GridObject,
    at_rest: bool,
}

impl GridPoint {
    fn update_point(&mut self, grid_object: GridObject, at_rest: bool) {
        self.occupied = grid_object;
        self.at_rest = at_rest;
    }
}
