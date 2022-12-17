use std::env;
use std::{fs, fmt::Display};
use std::path::Path;
use std::time::Instant;
use crate::solutions::*;
use clap::Parser;

pub mod domain;
pub mod solutions;

static ANSI_ITALIC: &str = "\x1b[3m";
static ANSI_BOLD: &str = "\x1b[1m";
static ANSI_RESET: &str = "\x1b[0m";

#[derive(Parser)]
struct RunArgument {
    pattern: String,
    day: i32
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
    let run_arguement = RunArgument::parse();
    env::set_var("RUST_BACKTRACE", "1");
    match run_arguement.day {
        2 => ship_it_extreme!(day02, load_file("day2_input.txt"), "Day 2"),
        3 => ship_it_extreme!(day03, load_file("day3_input.txt"), "Day 3"),
        4 => ship_it_extreme!(day04, load_file("day4_input.txt"), "Day 4"),
        5 => ship_it_extreme!(day05, load_file("day5_input.txt"), "Day 5"),
        6 => ship_it_extreme!(day06, load_file("day6_input.txt"), "Day 6"),
        7 => ship_it_extreme!(day07, load_file("day7_input.txt"), "Day 7"),
        8 => ship_it_extreme!(day08, load_file("day8_input.txt"), "Day 8"),
        9 => ship_it_extreme!(day09, load_file("day9_input.txt"), "Day 9"),
        10 => ship_it_extreme!(day10, load_file("day10_input.txt"), "Day 10"),
        11 => ship_it_extreme!(day11, load_file("day11_input.txt"), "Day 11"),
        12 => ship_it_extreme!(day12, load_file("day12_input.txt"), "Day 12"),
        13 => ship_it_extreme!(day13, load_file("day13_input.txt"), "Day 13"),
        14 => ship_it_extreme!(day14, load_file("day14_input.txt"), "Day 14"),
        15 => ship_it_extreme!(day15, load_file("day15_input.txt"), "Day 15"),
        16 => ship_it_extreme!(day16, load_file("day16_input.txt"), "Day 16"),
        _ => panic!("Day hasnt happened yet")
    }
}

fn load_file(path: &str) -> String {
    let file_path = Path::new(path);
    
    if !file_path.exists() {
        panic!("failure");
    }

    return fs::read_to_string(file_path).unwrap().to_string();
}