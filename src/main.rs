use crate::solutions::day01;

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
}