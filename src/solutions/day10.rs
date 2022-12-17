use std::collections::HashMap;

pub fn part_one(input: String) -> i32 { 
    let mut cpu = Cpu { current_instruction: Box::new(Noop { cycles: 0}), register: 1};
    let mut instructions:Vec<Instructions> = input.lines().map(|line| {
        let terms:Vec<&str> = line.split_whitespace().collect();
        let split = terms.as_slice();

        match split[0] {
            "addx" => Instructions::Addx(split[1].parse::<i32>().unwrap()),
            "noop" => Instructions::Noop,
            _ => panic!("Instruction not found!")
        }
    }).collect();

    let mut register_records:HashMap<i32,RegisterRecord> = HashMap::new();
    for x in 1..220 + 1 {
        if cpu.is_thread_free() && instructions.len() > 0 {
            cpu.push_instruction(instructions.get(0).unwrap());
            instructions.remove(0);
        }
        register_records.insert(x, RegisterRecord { register: cpu.register, cycle: x });
        cpu.run_cycle();
    }

    return sum_signal_strengths(register_records, [20,60,100,140,180,220].to_vec());

}

fn sum_signal_strengths(register_records: HashMap<i32, RegisterRecord>, interesting_cycles: Vec<i32>) -> i32 {

    let mut sum = 0;
    interesting_cycles.into_iter().for_each(|cycle| {
        let signal = register_records.get(&cycle).unwrap().get_signal_strength();
        sum = sum + signal;
    });

    return sum;
}

pub fn part_two(input: String) -> i32 { 

    let mut cpu = Cpu { current_instruction: Box::new(Noop { cycles: 0}), register: 1};
    let mut instructions:Vec<Instructions> = input.lines().map(|line| {
        let terms:Vec<&str> = line.split_whitespace().collect();
        let split = terms.as_slice();

        match split[0] {
            "addx" => Instructions::Addx(split[1].parse::<i32>().unwrap()),
            "noop" => Instructions::Noop,
            _ => panic!("Instruction not found!")
        }
    }).collect();

    let mut register_records:HashMap<i32,RegisterRecord> = HashMap::new();
    for cycle in 0..240 {
        if cpu.is_thread_free() && instructions.len() > 0 {
            cpu.push_instruction(instructions.get(0).unwrap());
            instructions.remove(0);
        }
        register_records.insert(cycle, RegisterRecord { register: cpu.register, cycle });
        if cycle % 40 == 0 {
            println!("");
        }
        print_pixel(cpu.register, cycle);

        cpu.run_cycle();
    }

    println!("");

    return 0;
}


fn print_pixel(sprite_position: i32, cycle: i32) {
    let crt_position = cycle % 40;
    let sprite_positions = [sprite_position - 1, sprite_position, sprite_position + 1].to_vec();
    if sprite_positions.contains(&crt_position) {
        print!("#");
    }
    else{
        print!(".");
    }
}

struct RegisterRecord {
    register: i32,
    cycle: i32
}

impl RegisterRecord {
    fn get_signal_strength(&self) -> i32 {
        return self.cycle * self.register;
    }
}


enum Instructions {
    Addx(i32),
    Noop
}

struct Cpu { 
    current_instruction: Box<dyn Program>,
    register: i32
}

impl Cpu {
    fn run_cycle(&mut self) {
        let (current_instruction, updated_register) = self.current_instruction.run_cycle(self.register);
        self.current_instruction = current_instruction;
        self.register = updated_register;
    }

    fn is_thread_free(&self) -> bool {
        return self.current_instruction.get_remaining_cycles() <= 0;
    }

    fn push_instruction(&mut self, instruction: &Instructions) {
        match instruction {
            Instructions::Addx(value) => self.current_instruction = Box::new(Addx::new(*value, 2)),
            Instructions::Noop => self.current_instruction = Box::new(Noop::new(1))
        }
    }
}

#[derive(Debug)]
struct Addx {
    increment_value: i32,
    cycles: i32
}

impl Addx {
    fn new(increment_value: i32, cycles: i32) -> Self {
        Addx { increment_value, cycles}
    }
}


impl Program for Addx {
    fn get_remaining_cycles(&self) -> i32 {
        return self.cycles;
    }
    fn run_cycle(&self, register: i32) -> (Box<dyn Program>,i32) {
        let cycles_left = self.cycles - 1;
        let mut updated_register = register;
        if cycles_left == 0 {
            updated_register = updated_register + self.increment_value;
        }
        return (Box::new(Addx { cycles: cycles_left, increment_value: self.increment_value }), updated_register);
    }
}

#[derive(Debug)]
struct Noop {
    cycles: i32
}

impl Noop {
    fn new(cycles: i32) -> Self {
        Noop { cycles }
    }
}

impl Program for Noop {
    fn get_remaining_cycles(&self) -> i32 {
        return self.cycles;
    }

    fn run_cycle(&self, register: i32) -> (Box<dyn Program>,i32) {
        return (Box::new(Noop { cycles: self.cycles - 1 }), register);
    }
}


trait Program: std::fmt::Debug { 
    fn get_remaining_cycles(&self) -> i32;
    fn run_cycle(&self, register: i32) -> (Box<dyn Program>,i32);
}
