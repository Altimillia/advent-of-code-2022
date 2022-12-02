use std::fs;
use std::path::Path;
use crate::solutions::{day01, day02};

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
}

fn load_file(path: &str) -> String {
    let file_path = Path::new(path);
    
    if !file_path.exists() {
        panic!("failure");
    }

    return fs::read_to_string(file_path).unwrap().to_string();
}