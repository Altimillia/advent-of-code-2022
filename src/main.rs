use std::env;
use std::{fs, fmt::Display};
use std::path::Path;
use std::time::Instant;
use crate::solutions::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12};

pub mod domain;
pub mod solutions;

static ANSI_ITALIC: &str = "\x1b[3m";
static ANSI_BOLD: &str = "\x1b[1m";
static ANSI_RESET: &str = "\x1b[0m";

macro_rules! ship_it {
    ($answer:expr, $day:expr) => {
        println!("-------");
        println!("{0}", $day);
        println!("{0}", $answer);
        println!("-------");
    };
}

fn print_result<T: Display>(func: impl FnOnce(String) -> T, input: String) {
    let timer = Instant::now();
    let result = func(input);
    let time = timer.elapsed();
    println!(
        "{} {}(elapsed: {:.2?}){}",
        result, ANSI_ITALIC, time, ANSI_RESET
    );
}

macro_rules! ship_it_extreme {
    ($day:path, $input:expr, $day_name:expr) => {{
        use $day::*;
        println!("----");
        println!("🎄 {}{}{} 🎄", ANSI_BOLD, $day_name, ANSI_RESET);
        println!("🎄 {}Part 1{} 🎄", ANSI_BOLD, ANSI_RESET);
        print_result(part_one, $input);
        println!("🎄 {}Part 2{} 🎄", ANSI_BOLD, ANSI_RESET);
        print_result(part_two, $input);
        println!("----");
    }};
}


fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());
    ship_it!(day01::part_one(), "day1a");
    ship_it!(day01::part_two(), "day1b");

    ship_it_extreme!(day02, load_file("day2_input.txt"), "Day 2");
    ship_it_extreme!(day03, load_file("day3_input.txt"), "Day 3");
    ship_it_extreme!(day04, load_file("day4_input.txt"), "Day 4");
    ship_it_extreme!(day05, load_file("day5_input.txt"), "Day 5");
    ship_it_extreme!(day06, load_file("day6_input.txt"), "Day 6");
    ship_it_extreme!(day07, load_file("day7_input.txt"), "Day 7");
    ship_it_extreme!(day08, load_file("day8_input.txt"), "Day 8");
    ship_it_extreme!(day09, load_file("day9_input.txt"), "Day 9");
    ship_it_extreme!(day10, load_file("day10_input.txt"), "Day 10");
    // ship_it_extreme!(day11, load_file("day11_input.txt"), "Day 11");
    ship_it_extreme!(day12, load_file("day12_input.txt"), "Day 12");
}

fn load_file(path: &str) -> String {
    let file_path = Path::new(path);
    
    if !file_path.exists() {
        panic!("failure");
    }

    return fs::read_to_string(file_path).unwrap().to_string();
}