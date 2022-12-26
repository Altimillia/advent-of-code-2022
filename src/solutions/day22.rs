use std::{collections::{HashMap, HashSet}, fmt::Display, ops::{Add, Sub}};
use nom::{IResult, character::{complete::{alpha1}}, multi::{many0}, branch::alt};
use num::integer::Roots;

use crate::{domain::{point::{*}, vertex::Vertex}, tools::{parse_numbers}};

static DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
pub fn new(x: f32, y: f32, z: f32) -> Self {
    return Vector3 { x, y, z };
}
    
}

impl From<Vertex> for Vector3 {
    fn from(p: Vertex) -> Self {
        Vector3::new(p.x as f32, p.y as f32, p.z as f32)
    }
}
impl From<Vector3> for Vertex {
    fn from(p: Vector3) -> Self {
        Vertex::new(p.x.round() as i32, p.y.round() as i32, p.z.round() as i32)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}



pub fn part_one(input: String) -> impl Display {
    let (grid, instructions) = parse_map(input);


    //println!("{:?}", grid);
    let (position, direction) = move_around_map(grid, instructions);

    (position.y * 1000) + (position.x * 4) + direction.direction_score()
}

pub fn part_two(input: String) -> impl Display {
    let (mut grid, instructions) = parse_map(input);
    let mut cube = add_cube(&mut grid);

    let (position, direction) = walk_cube(&grid, &mut cube, instructions);

    (position.y * 1000) + (position.x * 4) + direction.direction_score()
}

fn walk_cube(grid: &Grid, cube: &mut Cube, mut instructions: Vec<Instruction>) -> (Point, Direction) {
    println!("{}", &grid.get_starting_position());
    let pos3 = cube
        .get_3d_pos(&grid.get_starting_position())
        .ok_or("invalid 3d map").unwrap();

    let mut cube_walk = cube.clone();
    let mut visited = Vec::new();

    let mut player = Player3D { position3D: pos3, facing3D: Direction::East, position2d: grid.get_starting_position()  };
    while instructions.len() > 0 {
        let next_ins = instructions.remove(0);

        match next_ins {
            Instruction::Move(movement) => {
                let mut remaining = movement;

                while remaining > 0 {
                    let cube_orig = cube_walk.clone();

                    let mut next_pos3 = player.position3D + player.facing3D.to_vertex();
                    match next_pos3 {
                        p if p.x < 0 => {
                            cube.rotate(rad(90), 0.0, 0.0);
                            next_pos3.x = grid.side_len - 1;
                        }
                        p if p.x >= grid.side_len => {
                            cube.rotate(rad(-90), 0.0, 0.0);
                            next_pos3.x = 0;
                        }
                        p if p.y < 0 => {
                            cube.rotate(0.0, rad(-90), 0.0);
                            next_pos3.y = grid.side_len - 1;
                        }
                        p if p.y >= grid.side_len => {
                            cube.rotate(0.0, rad(90), 0.0);
                            next_pos3.y = 0;
                        }
                        _ => (),
                    };

                    let pos2 = cube.tiles.get(&next_pos3).unwrap();

                    let tile = grid.tiles.get(pos2).unwrap();

                    player.move_to_tile(next_pos3, *tile, cube);
                    if matches!(tile, Tile::Floor) {
                        remaining -= 1;
                        visited.push(player.position2d);
                    }
                    else {
                        remaining = 0;
                    }
                    cube_walk = cube_orig;
                }
            },
            Instruction::Rotate(rotation) => { 
                player.change_facing(rotation); 
            },
        }

    }

    let facing = DIRECTIONS
        .iter()
        .position(|&p| p == (player.position2d - visited[visited.len() - 2]))
        .ok_or("case when turning on the last tile is not handled");

    println!("{:?}", facing);
    println!("{:?}",  player.facing3D);
    (player.position2d, player.facing3D)
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


fn add_cube(grid: &mut Grid) -> Cube {
    // find position of sides within the tile map
    let mut sides = Vec::new();
    for y in 0..(grid.height / grid.side_len) {
        for x in 0..(grid.width / grid.side_len) {
            let tile_pos = Point::new((x * grid.side_len) + 1, (y * grid.side_len) + 1);
            println!("{:?}", tile_pos);
            if grid.tiles.contains_key(&tile_pos) {
                println!("{:?}", Point::new(x, y));
                sides.push(Point::new(x, y));
            }
        }
    }

    // map sides to cube
    let start = sides[0].clone();
    let sides_map: HashSet<Point> = HashSet::from_iter(sides);
    let mut cube = Cube {
        side_len: 50,
        tiles: HashMap::new()
    };
    fill_cube(&start, &sides_map, &mut HashSet::new(), &mut cube, &grid);
    return cube;
}

fn rad(deg: i32) -> f32 {
    (deg as f32).to_radians()
}

fn fill_cube(
    side: &Point,
    sides: &HashSet<Point>,
    visited: &mut HashSet<Point>,
    cube: &mut Cube,
    grid: &Grid
) {
    for y in side.y * cube.side_len..side.y * cube.side_len + cube.side_len {
        for x in side.x * cube.side_len..side.x * cube.side_len + cube.side_len {
            let from = Point::new(x + 1, y + 1);
            let to = Vertex::new(
                x - side.x * cube.side_len,
                grid.side_len - 1 - (y - side.y * cube.side_len), // flip y
                1,
            );
            cube.tiles.insert(to, from);
        }
    }

    for (dir_idx, dir) in DIRECTIONS.iter().enumerate() {
        let next_side = *side + *dir;
        if visited.insert(next_side) && sides.contains(&next_side) {
            match dir_idx {
                0 | 2 => cube.rotate(rad((dir_idx as i32 - 1) * 90), 0.0, 0.0),
                _ => cube.rotate(0.0, rad((dir_idx as i32 - 2) * 90), 0.0),
            }
            fill_cube(&next_side, sides, visited, cube, grid);
            match dir_idx {
                0 | 2 => cube.rotate(rad((dir_idx as i32 - 1) * -90), 0.0, 0.0),
                _ => cube.rotate(0.0, rad((dir_idx as i32 - 2) * -90), 0.0),
            }
        }
    }
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

#[allow(non_snake_case)]
struct Player3D {
    position3D: Vertex,
    facing3D: Direction,
    position2d: Point
}

impl Player3D {
    fn move_to_tile(&mut self, pos3d: Vertex, tile: Tile, cube: &Cube) {
        match tile {
            Tile::Floor => 
            { 
                self.position3D = pos3d;
                self.position2d = *cube.tiles.get(&pos3d).unwrap(); 
            },
            Tile::Wall => (),
        }
    }

    fn change_facing(&mut self, rotation: Rotation) { 
        match rotation {
            Rotation::Left => self.facing3D = self.facing3D.rotate_left(),
            Rotation::Right => self.facing3D = self.facing3D.rotate_right(),
        }
    }
}


#[derive(Debug, Clone)]
struct Cube {
    side_len: i32,
    tiles: HashMap<Vertex, Point>
}

impl Cube {
    fn get_3d_pos(&self, p: &Point) -> Option<Vertex> {
        self.tiles.iter().find(|(_, &p2)| p2 == *p).map(|x| *x.0)
    }

    #[allow(non_snake_case)]
    fn rotate(&mut self, pitch: f32, roll: f32, yaw: f32) {
        let cosa = yaw.cos();
        let sina = yaw.sin();
        let cosb = pitch.cos();
        let sinb = pitch.sin();
        let cosc = roll.cos();
        let sinc = roll.sin();
        let Axx = cosa * cosb;
        let Axy = cosa * sinb * sinc - sina * cosc;
        let Axz = cosa * sinb * cosc + sina * sinc;
        let Ayx = sina * cosb;
        let Ayy = sina * sinb * sinc + cosa * cosc;
        let Ayz = sina * sinb * cosc - cosa * sinc;
        let Azx = -sinb;
        let Azy = cosb * sinc;
        let Azz = cosb * cosc;

        let t = Vector3::new(
            (self.side_len as f32 - 1.0) / -2.0,
            (self.side_len as f32 - 1.0) / -2.0,
            (self.side_len as f32 - 1.0) / 2.0,
        );

        // translate
        let tiles_f = self.tiles.drain().map(|(p, c)| (Vector3::from(p) + t, c));

        // rotate
        let tiles_f = tiles_f.map(|(p, c)| {
            let p2 = Vector3::new(
                Axx * p.x + Axy * p.y + Axz * p.z,
                Ayx * p.x + Ayy * p.y + Ayz * p.z,
                Azx * p.x + Azy * p.y + Azz * p.z,
            );
            (p2, c)
        });

        // translate back
        self.tiles = HashMap::from_iter(tiles_f.map(|(p, c)| (Vertex::from(p - t), c)));
    }

}

#[derive(Debug)]
struct Grid {
    tiles: HashMap<Point, Tile>,
    side_len: i32,
    height: i32,
    width: i32
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut y_index = 1;
        input
            .lines()
            .for_each(|l| {
                let mut x_index = 1;
                l.chars().enumerate().for_each(|(_,c)| {
                    match c {
                        '.' => { tiles.insert(Point::new(x_index, y_index), Tile::Floor); },
                        '#' => { tiles.insert(Point::new(x_index, y_index), Tile::Wall); },
                        _ => ()
                    }
                    x_index += 1;
                });
                y_index += 1;
            });

        
            
        let side_len = (tiles.len() / 6).sqrt() as i32;
        Grid { tiles, side_len, height: 200, width: 150 }
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

    fn to_vertex(&self) -> Vertex {
        match self {
            Direction::North => Vertex { x: 0, y: -1, z:0 },
            Direction::East => Vertex { x: 1, y: 0, z: 0},
            Direction::South => Vertex { x: 0, y: 1, z: 0},
            Direction::West =>  Vertex { x: -1, y: 0, z: 0},
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