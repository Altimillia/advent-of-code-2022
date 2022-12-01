use std::{env, fs};
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


// fn day1a_puzzle() -> i32 {
//     let file_path = Path::new("day1_input.txt");
    
//     if !file_path.exists() {
//         panic!("failure");
//     }

//     let file_content = fs::read_to_string(file_path).unwrap();
//     let mut elves = Vec::<Elf>::new();
//     let inventories = file_content.split("\n\n");


//     inventories.enumerate().for_each(|(_pos, a)| {
//         let mut calories = 0;

//         a.lines().for_each(|f| {
//             calories += f.parse::<i32>().unwrap();
//         });

//         elves.push(Elf { total_calories: calories });
//     });
 

//     let answer = elves.iter().max_by_key(|x| x.total_calories);
//     return answer.unwrap().total_calories;
// }
