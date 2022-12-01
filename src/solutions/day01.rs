use std::fs;
use std::path::Path;
use crate::domain::elf::Elf;


pub fn part_one() -> i32 {
    let elves = get_elves();
    let answer = elves.iter().max_by_key(|x| x.total_calories);
    return answer.unwrap().total_calories;
}

pub fn part_two() -> i32 { 
    let mut elves = get_elves();
    elves.sort_by(|a,b| b.total_calories.cmp(&a.total_calories));
    return elves[0].total_calories + elves[1].total_calories + elves[2].total_calories;
}

fn get_elves() -> Vec::<Elf> {
    let file_path = Path::new("day1_input.txt");
    
    if !file_path.exists() {
        panic!("failure");
    }

    let file_content = fs::read_to_string(file_path).unwrap();
    let mut elves = Vec::<Elf>::new();
    let inventories = file_content.split("\n\n");


    inventories.enumerate().for_each(|(_pos, a)| {
        let mut calories = 0;

        a.lines().for_each(|f| {
            calories += f.parse::<i32>().unwrap();
        });

        elves.push(Elf { total_calories: calories });
    });

    return elves;
}