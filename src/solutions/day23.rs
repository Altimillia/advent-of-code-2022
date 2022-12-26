use core::panic;
use std::{fmt::Display, collections::HashMap, cmp, time::Instant};

use itertools::Itertools;

use crate::domain::point::Point;


static NORTH_DIRECTIONS: [Direction; 3] = [
    Direction::NorthEast,
    Direction::North,
    Direction::NorthWest
];

static SOUTH_DIRECTIONS: [Direction; 3] = [
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest
];

static EAST_DIRECTIONS: [Direction; 3] = [
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast
];

static WEST_DIRECTIONS: [Direction; 3] = [
    Direction::NorthWest,
    Direction::West,
    Direction::SouthWest
];

static ALL_DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest
];

pub fn part_one(input: String) -> impl Display {
    let mut grid = parse(input);
    //run_simulation(grid);

    let map = grid.get_print_string();
    //println!("{}", map);
    grid.get_empty_ground_in_elf_rectangle();
    grid = run_simulation(grid, 10);
    // println!("{}", grid.get_print_string());
    
    grid.get_empty_ground_in_elf_rectangle()
}

pub fn part_two(input: String) -> impl Display {
    let mut grid = parse(input);
    grid.get_empty_ground_in_elf_rectangle();
    let rounds = run_simulation_until(grid, 1000);
    
    rounds
}


fn run_simulation(mut grid: Grid, round_to_run: i32) -> Grid {
    let mut round_number = 1;
    let mut priority = vec![Direction::North, Direction::South, Direction::West, Direction::East];

    while round_number <= round_to_run {
        // Step One, decision.
        let grid_copy = grid.clone();
        let elf_positions:HashMap<Point, Elf> = grid_copy.elves.into_iter().map(|e| -> (Point, Elf) {(e.position, e)}).collect();
        for elf in grid.elves.iter_mut() {
            let proposed = elf.decide_quick(&elf_positions, priority.clone());
            elf.proposed_move = proposed;
        }

        let grid_copy = grid.clone();

        for elf in grid.elves.iter_mut() {
           
            if elf.proposed_move.is_none() {
                continue;
            }
            if grid_copy.elves.iter().filter(|e| e.position != elf.position).any(|e|  e.proposed_move == elf.proposed_move) {
                continue;
            }
            elf.move_to_spot(elf.proposed_move.unwrap());
        }


        for elf in grid.elves.iter_mut() {
            elf.end_of_round();
            
        }
        // println!("End of Round: {}", round_number);
        // println!("{}", grid.get_print_string());
        round_number += 1;
        priority.rotate_left(1);

    }

    grid
}

fn run_simulation_until(mut grid: Grid, round_to_run: i32) -> i32 {
    let mut round_number = 1;
    let mut priority = vec![Direction::North, Direction::South, Direction::West, Direction::East];

    while round_number <= round_to_run {

        let mut elf_moved = false;
        let grid_copy = grid.clone();
        let elf_positions:HashMap<Point, Elf> = grid_copy.elves.into_iter().map(|e| -> (Point, Elf) {(e.position, e)}).collect();
        for elf in grid.elves.iter_mut() {
            let proposed = elf.decide_quick(&elf_positions, priority.clone());
            elf.proposed_move = proposed;
        }

        let grid_copy = grid.clone();

        for elf in grid.elves.iter_mut() {
           
            if elf.proposed_move.is_none() {
                continue;
            }
            if grid_copy.elves.iter().filter(|e| e.position != elf.position).any(|e|  e.proposed_move == elf.proposed_move) {
                continue;
            }

            elf_moved = true;
            elf.move_to_spot(elf.proposed_move.unwrap());
        }

        for elf in grid.elves.iter_mut() {
            elf.end_of_round();
            
        }

        if !elf_moved {
            break;
        }
        round_number += 1;
        priority.rotate_left(1);

    }

    round_number
}


fn parse(input: String) -> Grid {
    let mut elves = Vec::new();
    let mut y_index = 100;
    input
        .lines()
        .for_each(|l| {
            let mut x_index = 100;
            l.chars().enumerate().for_each(|(n,c)| {
                match c {
                    '#' => { elves.push( 
                        Elf { 
                                position: Point { x: x_index, y: y_index },
                                proposed_move: None 
                            }); 
                        },
                    _ => ()
                }
                x_index += 1;
            });
            y_index += 1;
        });

    Grid { elves: elves }
}

#[derive(Clone, Debug)]
struct Grid {
    elves: Vec<Elf>
}

impl Grid {

    fn get_empty_ground_in_elf_rectangle(&self) -> i32 {
        let (min, max) = self.get_bounds_with_elves();

        let x_size = max.x - min.x + 1;
        let y_size = max.y - min.y + 1;
        let total_area = x_size * y_size;

        return total_area - self.elves.len() as i32;
    }
    fn get_bounds_with_elves(&self) -> (Point, Point) {
        let positions = self.elves.iter().map(|p| p.position).collect_vec();
        let min_x = positions.iter().map(|p| p.x).min().unwrap();
        let max_x = positions.iter().map(|p| p.x).max().unwrap();
        let min_y = positions.iter().map(|p| p.y).min().unwrap();
        let max_y = positions.iter().map(|p| p.y).max().unwrap();

        return (Point::new(min_x, min_y), Point::new(max_x, max_y));
    }

    fn get_print_string(&self) -> String {
        let positions = self.elves.iter().map(|p| p.position).collect_vec();
        let min_x = positions.iter().map(|p| p.x).min().unwrap();
        let max_x = positions.iter().map(|p| p.x).max().unwrap();
        let min_y = positions.iter().map(|p| p.y).min().unwrap();
        let max_y = positions.iter().map(|p| p.y).max().unwrap();

        let mut map = String::new();
        map.push_str("\n");
        map.push_str("   ");
        for x in (min_x - 2)..=(max_x + 2) { 
            map.push_str(format!("{} ", x.to_string()).as_str());
        }
        for y in (min_y - 2)..=(max_y + 2) {
            map.push_str("\n");
            map.push_str(format!("{} ", y.to_string()).as_str());
            
            for x in (min_x - 2)..=(max_x + 2) {
                map.push_str("  ");
                if positions.contains(&Point::new(x, y)) {
                    map.push_str("#");
                }
                else {
                    map.push_str(".");
                }
                map.push_str(" ");
            }
        }

        return map;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Elf {
    position: Point,
    proposed_move: Option<Point>
}

impl Elf {
    fn decide_quick(&self, elves: &HashMap<Point, Elf>, direction_priority: Vec<Direction>) -> Option<Point> {

        fn get_elves_in_directions(elves: &HashMap<Point, Elf>, origin_point: Point, directions: Vec<Direction>) -> bool {
            for dir in directions {
                if elves.contains_key(&(origin_point + dir.to_point())) {
                    return true;
                }
            }
            return false;
        }
        if !get_elves_in_directions(elves, self.position, ALL_DIRECTIONS.to_vec()) {
            return None; 
        }


        for dir in direction_priority {
            match dir {
                Direction::North => {
                    if get_elves_in_directions(elves, self.position, NORTH_DIRECTIONS.to_vec()) {
                        continue;
                    }

                    return Some(self.position + dir.to_point());
                },
                Direction::East => {
                    if get_elves_in_directions(elves, self.position, EAST_DIRECTIONS.to_vec()) {
                        continue;
                    }

                    return Some(self.position + dir.to_point());
                },
                Direction::South => {
                    if get_elves_in_directions(elves, self.position, SOUTH_DIRECTIONS.to_vec()) {
                        continue;
                    }

                    return Some(self.position + dir.to_point());
                },
                Direction::West => {
                    if get_elves_in_directions(elves, self.position, WEST_DIRECTIONS.to_vec()) {
                        continue;
                    }

                    return Some(self.position + dir.to_point());
                },
                Direction::NorthEast => panic!("Not a priortity!"),
                Direction::NorthWest => panic!("Not a priortity!"),
                Direction::SouthEast => panic!("Not a priortity!"),
                Direction::SouthWest => panic!("Not a priortity!"),
            }
        }

        None
        // 
    }

    fn move_to_spot(&mut self, new_position: Point) {


        //println!("Elf {} to {}", self.position, new_position);
        self.position = new_position;
    }

    fn end_of_round(&mut self) {
        self.proposed_move = None;
    }
}


#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest  
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Direction::North => Point { x: 0, y: -1 },
            Direction::East => Point { x: 1, y: 0},
            Direction::South => Point { x: 0, y: 1 },
            Direction::West =>  Point { x: -1, y: 0},
            Direction::NorthEast => Point { x: 1, y: -1},
            Direction::NorthWest => Point { x: -1, y: -1},
            Direction::SouthEast => Point { x: 1, y: 1},
            Direction::SouthWest => Point { x: -1, y: 1}
        }
    }


    fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::SouthEast,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
        }
    }

}