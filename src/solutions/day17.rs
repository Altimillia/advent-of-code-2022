use core::panic;
use std::{fmt::Display, collections::HashMap, hash::Hash};

use itertools::Itertools;

use crate::domain::point::Point;

const MAX_X: i32 = 6;
const MIN_X: i32 = 0;

pub fn part_one(input: String) -> impl Display {
    let jet_pattern = JetPattern::parse(input);

    tetris_time(400 , jet_pattern);

    
    0
}

pub fn part_two(input: String) -> impl Display {
    let jet_pattern = JetPattern::parse(input);

    let height_delta = tetris_time(15000 , jet_pattern);
    const INITIAL_PATTERN_SKIP_LEN: usize = 2500;
    let height_delta_for_pattern = &height_delta[INITIAL_PATTERN_SKIP_LEN..];
    let mut found_pattern_len = 0;
    for pattern_len in 1..=height_delta_for_pattern.len() / 2 {
        let pattern = &height_delta_for_pattern[0..pattern_len];
        let mut found = true;
        for i in 0..height_delta_for_pattern.len() - pattern_len {
            if height_delta_for_pattern[i + pattern_len] != pattern[i % pattern_len] {
                found = false;
                break;
            }
        }
        if found {
            found_pattern_len = pattern_len;
            break;
        }
    }
    assert!(found_pattern_len > 0);
    const NUM_SHAPES: u64 = 1000000000000;
    let pattern = &height_delta_for_pattern[0..found_pattern_len];
    let pattern_sum = pattern.iter().sum::<u64>();
    let initial_deltas = &height_delta[0..height_delta.len() / 4];
    let initial_sum = initial_deltas.iter().sum::<u64>();
    let num_patterns = (NUM_SHAPES - initial_deltas.len() as u64) / pattern.len() as u64;
    let num_leftover = ((NUM_SHAPES - initial_deltas.len() as u64) % pattern.len() as u64) as usize;
    let leftover_sum = pattern[0..num_leftover].iter().sum::<u64>();

    (initial_sum + pattern_sum * num_patterns + leftover_sum).to_string()
}

fn get_next_rock(rock_number: i32) -> Rock {

    let remainder = rock_number % 5;
    return match remainder {
        0 => Rock::dash_shape(),
        1 => Rock::cross_shape(),
        2 => Rock::l_shape(),
        3 => Rock::tall_shape(),
        4 => Rock::square_shape(),
        _ => panic!("Improbable")
    };
}


fn tetris_time(total_rock_count: i32, jet_pattern: JetPattern) -> Vec<u64> {
    let mut grid = Grid { tiles: HashMap::new() };

    let mut current_rock_count:i32 = 0;
    let mut tick = 0;
    let jet_index_length = jet_pattern.pattern.len();
    let mut height_delta: Vec<u64> = Vec::with_capacity(total_rock_count as usize);

    let mut prev_height = 0;

    while current_rock_count < total_rock_count {
        let mut rock = get_next_rock(current_rock_count);


        let high_point = grid.get_highest_rock_or_floor();
            
        grid.expand_upwards_to_row(high_point + 3 + rock.height);
        if high_point != 0 {
            
            height_delta.push((high_point - prev_height) as u64);
        }
        rock = rock.shift_rock_to_position(get_rock_start(&grid, &rock));
        prev_height = high_point;

        loop {
            let direction = jet_pattern.get_pattern_for_tick(tick);
            tick = tick + 1;

            let (shifted_rock, _) = shift_rock(rock, direction.get_direction_point(), &grid);
            rock = shifted_rock;

            let (downward_move_rock, moved_rock) = shift_rock(rock, Point::new(0,-1), &grid);
            rock = downward_move_rock;


            if !moved_rock { 
                grid.set_tiles_to_rock(&rock.points);
                break;
            }
        }

        current_rock_count = current_rock_count + 1;
    }

    println!("Highest {}", grid.get_highest_rock_or_floor());
    return height_delta;
    
}

fn get_rock_start(grid: &Grid, rock: &Rock) -> Point {
    let high_point = grid.get_highest_rock_or_floor() - 1;

    return Point::new(2, high_point + 4);
}

fn print_current_grid(grid: &Grid, rock: &Rock) {
    let current_max_height = grid.tiles.iter().map(|hm| hm.0.y).max().unwrap();
    //print_points(rock);
    println!("");
    
    for y in (0..=current_max_height).rev() {
        for x in MIN_X..=MAX_X {
            let point = &Point::new(x, y);
            if rock.points.contains(point) {
                print!("@");
            }
            else {
                match grid.tiles.get(point).unwrap() {
                    Tile::Air => print!("."),
                    Tile::Rock => print!("#"),
                    Tile::Floor => print!("-"),
                }
        }
        }
        println!("");
    }


}

fn shift_rock(rock: Rock, direction: Point, grid: &Grid) -> (Rock, bool) {
    
    let rock_shift = Rock { points: rock.points.iter().map(|p| *p + direction).collect_vec(), height: rock.height };
    if grid.can_fit(&rock_shift.points) {
        return (rock_shift, true);
    }

    return (rock, false);
}

fn print_points(rock: &Rock) {
    println!("Rock Position: ");
    rock.points.iter().for_each(|p| print!("{} ", p));
}


struct Grid {
    tiles: HashMap<Point, Tile>
}

impl Grid {
    fn expand_upwards_to_row(&mut self, row: i32) {
        let current_max_height = self.tiles.iter().map(|hm| hm.0.y).max().unwrap_or(-1);
        
        for x in MIN_X..=MAX_X {
            for y in current_max_height + 1..row {
                self.tiles.insert(Point::new(x, y), Tile::Air);
            }
        }
    }

    fn set_tiles_to_rock(&mut self, points: &Vec<Point>) {
        for p in points { 
            self.tiles.entry(*p).and_modify(|tile| *tile = Tile::Rock);
        }
    }

    fn get_highest_rock_or_floor(&self) -> i32 {
        self.tiles.iter().filter(|hm| matches!(hm.1, Tile::Rock) || matches!(hm.1, Tile::Floor)).map(|hm| hm.0.y).max().unwrap_or(-1) + 1
    }
    
    fn can_fit(&self, points: &Vec<Point>) -> bool {
        return points.iter().fold(true, |acc,p| { 
            if self.tiles.contains_key(p) {
                return match self.tiles[p] {
                    Tile::Floor => acc && false,
                    Tile::Air => acc,
                    Tile::Rock => acc && false,
                };
            }

            return acc && false;
        });
    }

    fn get_rock_start_position(&self) -> Point {
        let max_height = self.tiles.iter().map(|hm| hm.0.y).max().unwrap_or(0);

        return Point::new(MIN_X + 2, max_height);
    }
}

enum Tile {
    Air,
    Rock,
    Floor
}

struct Rock {
    points: Vec<Point>,
    height: i32
}

impl Rock {
    fn dash_shape() -> Self {
        return Rock { points: vec![Point::new(0,0), Point::new(1,0), Point::new(2,0), Point::new(3,0)], height: 1};
    }

    fn cross_shape() -> Self {
        return Rock { points: vec![Point::new(0,1), Point::new(1,0), Point::new(1,1), Point::new(2,1), Point::new(1,2)], height: 3 }
    }

    fn l_shape() -> Self {
        return Rock { points: vec![Point::new(2,2), Point::new(2,1), Point::new(0,0), Point::new(1,0), Point::new(2,0)], height: 3 }
    }

    fn tall_shape() -> Self {
        return Rock { points: vec![Point::new(0,0), Point::new(0,1), Point::new(0,2), Point::new(0,3)], height: 4 }
    }

    fn square_shape() -> Self {
        return Rock { points: vec![Point::new(0,0), Point::new(0,1), Point::new(1,0), Point::new(1,1)] , height: 3}
    }

    fn shift_rock_to_position(&self, position: Point) -> Self {
        
        return Rock { points: self.points.iter().map(|p| *p + position).collect_vec(), height: self.height};
    }


}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WindDirection {
    Left,
    Right
}

impl WindDirection {
    fn get_direction_point(&self) -> Point {
        match self {
            WindDirection::Left => Point::new(-1, 0),
            WindDirection::Right => Point::new(1, 0),
        }
    }
}

struct JetPattern {
    pattern: Vec<WindDirection>
}

impl JetPattern {
    fn get_pattern_for_tick(&self, tick: usize) -> WindDirection {
        let index = tick % self.pattern.len();
        return self.pattern[index];
    }

    fn parse(input: String) -> Self {
        let pattern = input.trim().chars().into_iter().map(|c| {
            match c { 
                '<' => WindDirection::Left,
                '>' => WindDirection::Right,
                _ => panic!("Not allowed input")
            }
        }).collect_vec();

        JetPattern { pattern: pattern }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{solutions::day17::Tile, domain::point::Point};

    use super::{get_next_rock, JetPattern, WindDirection, Grid, Rock};


    #[test]
    fn rock_shape_gets_right_rock_based_on_number() {
        let rock = get_next_rock(0);

        assert_eq!(rock.points.len(), 4);
    }

    #[test]
    fn jet_pattern_gets_a_wind_direction_by_tick() {
        let jet = JetPattern { pattern: vec![WindDirection::Left, WindDirection::Left, WindDirection::Right]};

        assert_eq!(jet.get_pattern_for_tick(0), WindDirection::Left);
        assert_eq!(jet.get_pattern_for_tick(1), WindDirection::Left);
        assert_eq!(jet.get_pattern_for_tick(2), WindDirection::Right);
        assert_eq!(jet.get_pattern_for_tick(3), WindDirection::Left);
        assert_eq!(jet.get_pattern_for_tick(4), WindDirection::Left);
        assert_eq!(jet.get_pattern_for_tick(5), WindDirection::Right);
        assert_eq!(jet.get_pattern_for_tick(6), WindDirection::Left);

    }

    #[test]
    fn grid_can_get_the_highest_rock() {
        let mut grid = Grid { tiles: HashMap::new()};

        let result = grid.get_highest_rock_or_floor();

        assert_eq!(result, 0);

        grid.tiles.insert(Point::new(0,6), Tile::Rock);

        grid.tiles.insert(Point::new(0,7), Tile::Air);
        assert_eq!(grid.get_highest_rock_or_floor(), 7);
    }

    #[test]
    fn grid_can_expand_upwards_multiple_rows() {
        let mut grid = Grid { tiles: HashMap::new()};

        grid.expand_upwards_to_row(7);
        assert_eq!(grid.tiles.len(), 7 * 7);
    }

    #[test]
    fn jet_pattern_parses_directions() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let jet = JetPattern::parse(input.to_string());

        assert_eq!(jet.get_pattern_for_tick(0), WindDirection::Right);
        assert_eq!(jet.get_pattern_for_tick(1), WindDirection::Right);
        assert_eq!(jet.get_pattern_for_tick(3), WindDirection::Left);
        assert_eq!(jet.get_pattern_for_tick(40), WindDirection::Right);
        assert_eq!(jet.get_pattern_for_tick(41), WindDirection::Right);
        assert_eq!(jet.get_pattern_for_tick(42), WindDirection::Right);
        assert_eq!(jet.get_pattern_for_tick(43), WindDirection::Left);
    }

    #[test]
    fn shift_rock_moves_to_a_new_position() {

        let rock = Rock::dash_shape();

        let new_rock = rock.shift_rock_to_position(Point::new(2, 3));

        assert_eq!(*new_rock.points.get(0).unwrap(), Point::new(2, 3));
        assert_eq!(*new_rock.points.get(3).unwrap(), Point::new(5, 3));
    }

    #[test]
    fn can_fit_allows_rocks_to_move_in_grid() {
        let rock = Rock::dash_shape();
        let mut grid = Grid { tiles: HashMap::new()};

        grid.expand_upwards_to_row(4);
        assert_eq!(grid.can_fit(&rock.shift_rock_to_position(Point::new(2, 3)).points), true);

    }
    
}