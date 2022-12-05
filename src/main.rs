use std::fs;
use std::path::Path;
use crate::solutions::{day01, day02, day03, day04};

pub mod domain;
pub mod solutions;

macro_rules! ship_it {
    ($answer:expr, $day:expr) => {
        println!("-------");
        println!("{0}", $day);
        println!("{0}", $answer);
        println!("-------");
    };
}

fn main() {
    ship_it!(day01::part_one(), "day1a");
    ship_it!(day01::part_two(), "day1b");
    ship_it!(day02::part_one(load_file("day2_input.txt")), "day2a");
    ship_it!(day02::part_two(load_file("day2_input.txt")), "day2b");
    ship_it!(day03::part_one(load_file("day3_input.txt")), "day3a");
    ship_it!(day03::part_two(load_file("day3_input.txt")), "day3b");
    ship_it!(day04::part_one(load_file("day4_input.txt")), "day4a");
    ship_it!(day04::part_two(load_file("day4_input.txt")), "day4b");
}

fn load_file(path: &str) -> String {
    let file_path = Path::new(path);
    
    if !file_path.exists() {
        panic!("failure");
    }

    return fs::read_to_string(file_path).unwrap().to_string();
}