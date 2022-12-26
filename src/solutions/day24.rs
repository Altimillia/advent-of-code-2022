use std::{fmt::Display, collections::{HashSet}};

use itertools::Itertools;

use crate::domain::point::Point;

pub fn part_one(input: String) -> impl Display {
    let grid_state = GridState::parse(input);
    println!("{}", grid_state.print_grid());
    let grid_state = grid_state.add_minutes(1);
    println!("{}", grid_state.print_grid());
    let grid_state = grid_state.add_minutes(1);
    println!("{}", grid_state.print_grid());
    0
}

pub fn part_two(_input: String) -> impl Display {
    0
}

#[derive(Debug, Clone)]
struct GridState {
    minutes: i32,
    blizzards: Vec<Blizzard>,
    walls: HashSet<Point>,
    size: Point
}

impl GridState {
    fn add_minutes(&self, minutes_to_add: i32) -> Self {
        let mut next_state = self.clone();

        // move blizzards
        let mut count = next_state.minutes;
        let target = next_state.minutes + minutes_to_add;
        
        while count < target {
            for blizzard in next_state.blizzards.iter_mut() {
                let mut next_pos = blizzard.position + blizzard.direction.to_point();
                if self.walls.contains(&next_pos) {
                    if next_pos.x == self.size.x {
                        next_pos = Point::new(1, next_pos.y);
                    }
                    else if next_pos.x == 0 {
                        next_pos = Point::new(self.size.x - 1, next_pos.y);
                    }
                    else if next_pos.y == self.size.y {
                        next_pos = Point::new(next_pos.x, 1);
                    }
                    else if next_pos.y == 0 {
                        next_pos = Point::new(next_pos.x, self.size.y - 1);
                    }
                }

                blizzard.position = next_pos;
                
            }

            count += 1;
        }

        next_state.minutes = target;

        return next_state;
    }
    fn print_grid(&self) -> String {

        let mut map = String::new();
        map.push_str("\n");
        map.push_str("   ");

        for y in 0..=self.size.y {
            map.push_str("\n");
            
            for x in 0..=self.size.x {
                if self.walls.contains(&Point::new(x, y)) {
                    map.push_str("#");
                    continue;
                }

                let blizzards = self.blizzards.iter().filter(|b| b.position == Point::new(x, y)).collect_vec();
                if blizzards.len() == 1 {
                    match blizzards[0].direction {
                        Direction::North => map.push_str("^"),
                        Direction::East => map.push_str(">"),
                        Direction::South => map.push_str("v"),
                        Direction::West => map.push_str("<"),
                    }
                }
                else if blizzards.len() > 1 {
                    map.push_str(blizzards.len().to_string().as_str());
                }
                else {
                    map.push_str(".")
                }
            }
        }

        return map;
    }
    fn parse(input: String) -> Self {
        let mut blizzards = Vec::new();
        let mut walls = HashSet::new();

        let mut y_index = 0;
        input
            .lines()
            .for_each(|l| {
                let mut x_index = 0;
                l.chars().enumerate().for_each(|(_,c)| {
                    match c {
                        '#' => { 
                            walls.insert(Point::new(x_index, y_index));
                            },
                        '>' => blizzards.push(Blizzard { direction: Direction::East, position: Point::new(x_index, y_index)}),
                        '<' => blizzards.push(Blizzard { direction: Direction::West, position: Point::new(x_index, y_index)}),
                        '^' => blizzards.push(Blizzard { direction: Direction::North, position: Point::new(x_index, y_index)}),
                        'v' => blizzards.push(Blizzard { direction: Direction::South, position: Point::new(x_index, y_index)}),
                        _ => ()
                    }
                    x_index += 1;
                });
                y_index += 1;
            });

        let max_x = walls.iter().map(|point| point.x).max().unwrap();
        let max_y = walls.iter().map(|point| point.y).max().unwrap();
    
        GridState { minutes: 0, blizzards: blizzards, walls: walls, size: Point::new(max_x,max_y) }
    }
}

#[derive(Debug, Clone)]
struct Blizzard {
    position: Point,
    direction: Direction
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North = 3,
    East = 0,
    South = 1,
    West = 2
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Direction::North => Point { x: 0, y: -1 },
            Direction::East => Point { x: 1, y: 0},
            Direction::South => Point { x: 0, y: 1 },
            Direction::West =>  Point { x: -1, y: 0},
        }
    }
}
