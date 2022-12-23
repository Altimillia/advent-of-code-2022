use std::{collections::HashMap, fmt::Display};
use nom::{IResult, character::{complete::{alpha1}}, multi::{many0}, branch::alt};

use crate::{domain::point::{*, self}, tools::{parse_numbers}};

pub fn part_one(input: String) -> impl Display {
    let (grid, instructions) = parse_map(input);


    //println!("{:?}", grid);
    let (position, direction) = move_around_map(grid, instructions);

    (position.y * 1000) + (position.x * 4) + direction.direction_score()
}

pub fn part_two(input: String) -> impl Display {
    let (grid, instructions) = parse_map(input);
    let cube = Cube::parse(grid, 50);
    0
}

fn move_around_map(grid: Grid, mut instructions: Vec<Instruction>) -> (Point, Direction) {
    let start = grid.get_starting_position();
    let mut player = Player { facing: Direction::East, position: start };

    println!("{:?}", player);

    while instructions.len() > 0 {
        let next_ins = instructions.remove(0);

        match next_ins {
            Instruction::Move(movement) => {
                let mut remaining = movement;

                while remaining > 0 {
                    let (point, tile) = grid.get_next_tile_in_direction(player.position, player.facing);
                    //println!("{:?} {:?}", point, tile);
                    player.move_to_tile(point, tile);
                    if matches!(tile, Tile::Floor) {
                        player.move_to_tile(point, tile);
                        remaining -= 1;
                    }
                    else {
                        remaining = 0;
                    }
                }
            },
            Instruction::Rotate(rotation) => player.change_facing(rotation),
        }

        //println!("{:?}", player);
    }

    return (player.position, player.facing)

}

fn parse_map(input: String) -> (Grid, Vec<Instruction>) {
    let mut s = input.split("\n\n");
    
    
    return (
        Grid::parse(s.next().unwrap()), 
        parse_instructions(s.next().unwrap()).unwrap().1
    );

}

fn parse_instructions(input: &str) -> IResult<&str,Vec<Instruction>> {
    many0(parse_instruction)(input)
    
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> { 
    alt((parse_move, parse_rotate))(input)
}

fn parse_move(input: &str) -> IResult<&str, Instruction> { 
    let (input, num) = (parse_numbers)(input)?;
    Ok((input, Instruction::Move(num)))
}

fn parse_rotate(input: &str) -> IResult<&str, Instruction> { 

    let (input, rotate_char) = (alpha1)(input)?;
    let ins = match rotate_char {
        "R" => Instruction::Rotate(Rotation::Right),
        "L" => Instruction::Rotate(Rotation::Left),
        _ => unreachable!()

    };
    Ok((input, ins))
}

#[derive(Debug)]
struct Player {
    position: Point,
    facing: Direction
}
impl Player {
    fn move_to_tile(&mut self, point: Point, tile: Tile) {
        match tile {
            Tile::Floor => self.position = point,
            Tile::Wall => (),
        }
    }

    fn change_facing(&mut self, rotation: Rotation) { 
        match rotation {
            Rotation::Left => self.facing = self.facing.rotate_left(),
            Rotation::Right => self.facing = self.facing.rotate_right(),
        }
    }
}

struct Cube {

    cube_faces: HashMap<CubeFace, Vec<Point>>
}

impl Cube {
    fn parse(grid: Grid, cube_size: i32) -> Self {
        let start = grid.get_starting_position();

        let mut cube_faces = HashMap::new();
        //
        cube_faces.insert(CubeFace::Top, vec![start, start + Point::new(cube_size, 0), start + Point::new(cube_size, cube_size), start + Point::new(0, cube_size)]);
        cube_faces.insert(CubeFace::Right, vec![start + Point::new(cube_size, 0), start + Point::new(cube_size * 2, 0), start + Point::new(cube_size * 2, cube_size), start + Point::new(cube_size, cube_size)]);

        Cube { cube_faces }
    }
}

#[derive(Debug)]
struct Grid {
    tiles: HashMap<Point, Tile>
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut y_index = 1;
        input
            .lines()
            .for_each(|l| {
                let mut x_index = 1;
                l.chars().enumerate().for_each(|(n,c)| {
                    match c {
                        '.' => { tiles.insert(Point::new(x_index, y_index), Tile::Floor); },
                        '#' => { tiles.insert(Point::new(x_index, y_index), Tile::Wall); },
                        _ => ()
                    }
                    x_index += 1;
                });
                y_index += 1;
            });

        Grid { tiles }
    }

    fn get_starting_position(&self) -> Point {
        let min_x = self.tiles.keys().filter(|key| key.y == 1).map(|key| key.x).min().unwrap();

        return Point { x: min_x, y: 1 }
    }

    fn get_next_tile_in_direction(&self, current_pos: Point, direction: Direction) -> (Point, Tile) {

        let next_position = current_pos + direction.to_point();

        if self.tiles.contains_key(&next_position) {
            return (next_position, self.tiles[&next_position]);
        }

        let edge_position = self.get_edge_position(current_pos, direction.reverse()).unwrap();

        return (*edge_position, self.tiles[&edge_position]);
    }

    fn get_edge_position(&self, position: Point, direction: Direction) -> Option<&Point> {
        match direction {
            Direction::North => {
                self.tiles.keys().filter(|k| k.x == position.x).min_by_key(|k| k.y)
            },
            Direction::East => {
                self.tiles.keys().filter(|k| k.y == position.y).max_by_key(|k| k.x)
            },
            Direction::South => {
                self.tiles.keys().filter(|k| k.x == position.x).max_by_key(|k| k.y)
            },
            Direction::West => {
                self.tiles.keys().filter(|k| k.y == position.y).min_by_key(|k| k.x)
            },
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Tile {
    Floor,
    Wall
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

    fn direction_score(&self) -> i32 {
        *self as i32
    }
  
    fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn rotate_left(&self) -> Self {
        match &self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn rotate_right(&self) -> Self {
        match &self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
#[derive(Debug)]
enum Instruction {
    Move(i32),
    Rotate(Rotation)
}


#[derive(Debug)]
enum Rotation {
    Left,
    Right
}

#[derive(PartialEq, Eq, Hash)]
enum CubeFace {
    Top,
    Down,
    Forward,
    Back,
    Left,
    Right
}

impl CubeFace {
    fn get_neighbor_face(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => todo!(),
            Direction::East => todo!(),
            Direction::South => todo!(),
            Direction::West => todo!(),
        }
    }

    fn get_north_neighbor(&self) -> Self {
        match &self {
            CubeFace::Top => CubeFace::Back,
            CubeFace::Down => CubeFace::Forward,
            CubeFace::Forward => CubeFace::Top,
            CubeFace::Back => CubeFace::Back,
            CubeFace::Left => todo!(),
            CubeFace::Right => todo!(),
        }

    }
}