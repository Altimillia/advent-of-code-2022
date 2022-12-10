use std::collections::HashSet;

use crate::domain::point::Point;

const NORTH: Point = Point { x: 0, y: 1};
const SOUTH: Point = Point { x: 0, y: -1};
const EAST:Point = Point { x: 1, y: 0 };
const WEST:Point = Point { x: -1, y: 0 };

pub fn part_one(input: String) -> usize { 

    let instructions:Vec<Instruction> = input
        .lines()
        .into_iter()
        .map(|f| Instruction::new(f))
        .collect();  


    return get_unique_positions_of_rope(instructions, 2);
}

pub fn part_two(input: String) -> usize {
    let instructions:Vec<Instruction> = input
        .lines()
        .into_iter()
        .map(|f| Instruction::new(f))
        .collect();  


    return get_unique_positions_of_rope(instructions, 10);
}

fn get_unique_positions_of_rope(instructions: Vec<Instruction>, number_of_knots: i32) -> usize { 
    let mut rope = Rope::new(number_of_knots);
    let movement_records:Vec<MovementRecord> = instructions
        .iter()
        .map(|instruction| {

            return rope.run_instruction(instruction);
        }).flatten().collect();

    let unique_positions = movement_records.iter().map(|rec| rec.tail_position).collect::<HashSet<Point>>();


    return unique_positions.len();
}

fn print_out_area(movement_records:Vec<MovementRecord>) { 
    let min = movement_records.iter().map(|f| f.knots.to_vec()).flatten().min().unwrap();
    let max = movement_records.iter().map(|f| f.knots.to_vec()).flatten().max().unwrap();

    let tail_points:Vec<Point> = movement_records.iter().map(|p| p.tail_position).collect();

    for x in min.x..max.x {
        let mut row:String = "".to_string();
        for y in min.y..max.y {
            if tail_points.contains(&Point::new(x, y)) {
                row = row.to_owned() + "#";
            }
            else {
                row = row.to_owned() + ".";
            }
        }
        println!("{}", row);
    }
}



struct Instruction {
    direction: Point,
    magnitude: i32
}
impl Instruction {
    pub fn new(instruction_line: &str) -> Self { 
        let terms:Vec<&str> = instruction_line.split_whitespace().collect();
        let split = terms.as_slice();

        let dir = match split[0] {
            "D" => SOUTH,
            "U" => NORTH,
            "R" => EAST,
            "L" => WEST,
            _ => panic!("Not a direction!")
        };
        let magnitude = split[1].parse::<i32>().unwrap();

        return Instruction { direction: dir, magnitude: magnitude };
    }
}

struct Rope {
    knots: Vec<Point>
}

impl Rope {

    fn new(number_of_knots: i32) -> Self {
        let mut knots:Vec<Point> = Vec::new();
        
        for _x in 0..number_of_knots {
            knots.push(Point::new(0,0));
        }

        return Rope { knots }
    }

    fn run_instruction(&mut self, instruction: &Instruction) -> Vec<MovementRecord> {

        let mut movement_records:Vec<MovementRecord> = Vec::new();

        for _x in 0..instruction.magnitude {
            self.knots[0] = self.knots[0] + instruction.direction;
            let mut rope_index = 1;

            while rope_index < self.knots.len() {
                self.update_tail(rope_index,self.knots[rope_index - 1]);
                rope_index = rope_index + 1;
            }
            
            movement_records.push(MovementRecord { knots: self.knots.to_vec(), tail_position: *self.knots.last().unwrap() });
        }
        return movement_records;
    }

    fn update_tail(&mut self, knot_to_update: usize, attached_to_knot: Point) {

        let diff = attached_to_knot - self.knots[knot_to_update];

        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            self.knots[knot_to_update] = self.knots[knot_to_update] + diff.normalize();  
        }
    }
}

#[derive(Clone)]
struct MovementRecord {
    knots: Vec<Point>,
    tail_position: Point
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::domain::point::Point;

    use super::{Rope, Instruction, NORTH, MovementRecord};


    #[test]
    fn rope_can_move_multiple_units() {
        let mut rope = Rope::new(2);
        

        rope.run_instruction(&Instruction { direction: NORTH, magnitude: 2});

        assert_eq!(rope.knots[0], Point::new(0, 2));
    }

    #[test]
    fn rope_can_move_one_unit() {
        let mut rope = Rope::new(2);
        

        rope.run_instruction(&Instruction { direction: NORTH, magnitude: 1});

        assert_eq!(rope.knots[0], Point::new(0, 1));
    }

    #[test]
    fn rope_can_run_test_scenario() {
        let input = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";

        let instructions:Vec<Instruction> = input
            .lines()
            .into_iter()
            .map(|f| Instruction::new(f.trim()))
            .collect();  
        let mut rope = Rope::new(2);

        let movement_records:Vec<MovementRecord> = instructions
            .iter()
            .map(|instruction| {
        
                return rope.run_instruction(instruction);
            }).flatten().collect();
        
        let unique_positions = movement_records.iter().map(|rec| 
            {
                return rec.tail_position
            }).collect::<HashSet<Point>>();

        assert_eq!(unique_positions.len(), 13);

    }
}