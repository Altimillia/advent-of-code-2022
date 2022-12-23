use core::panic;
use std::{fmt::Display, collections::HashMap};

use itertools::Itertools;
use nom::{bytes::{complete::{take_until, tag}, streaming::take}, IResult, branch::alt, character::complete::{one_of, space1}, sequence::{tuple, preceded}};
use crate::tools::parse_numbers_i64;

pub fn part_one(input: String) -> impl Display {
    let monkies = parse_monkies(input);
    monkey_math(monkies)
}

pub fn part_two(input: String) -> i64 {
    let monkies = parse_monkies(input);
    let mut monkey_solutions = HashMap::new();
    fill_solutions(&mut monkey_solutions, &monkies, "root".to_string());

    let monkey_map:HashMap<&str, MonkeyJob> = monkies.iter().map(|m| (m.id.as_str(), m.job.clone())).collect();

    let (mut unknown, mut result, mut correction) = ("root".to_string(), 0, -1);
    while unknown != "humn" {
        let MonkeyJob::Operation(left, op, right) = monkey_map[unknown.as_str()].clone() else {
            panic!();
        };

        (unknown, result) = match (monkey_solutions.get(&left), monkey_solutions.get(&right)) {
            (None, Some(&val)) => (left, op.solve_for_left(result, val)),
            (Some(&val), None) => (right, op.solve_for_right(result, val)),
            _ => panic!(),
        };

        // Change the result of the root equation to be negative
        result *= correction;
        correction = 1;
    }

    result
}

fn fill_solutions(monkey_solutions: &mut HashMap<String, i64>, monkies: &Vec<Monkey>, monkey: String) -> Option<i64> {
    if monkey == "humn" {
        return None;
    }

    let val = match monkies.iter().find(|x| x.id == monkey) {
        Some(found_monkey) => {
            match found_monkey.job.clone() {
                MonkeyJob::Number(num) => num,
                MonkeyJob::Operation(left_monkey, operator, right_monkey) => {
                    let left = fill_solutions(monkey_solutions, monkies, left_monkey);
                    let right = fill_solutions(monkey_solutions, monkies, right_monkey);
                    operator.eval(left?, right?)
                },
            }
        }
        _ => panic!("Where my monkey at?"),
    };

    monkey_solutions.insert(monkey, val);
    Some(val)
}


fn parse_monkies(input: String) -> Vec<Monkey> {
    input
        .lines()
        .map(|l| 
            Monkey::parse(l).unwrap().1
        ).collect_vec()
}

fn monkey_math(monkies: Vec<Monkey>) -> i64 {
    let mut monkey_solutions:HashMap<String, i64> = HashMap::new();
    while !monkey_solutions.contains_key("root") {
        let remaining_monkies = monkies.iter().filter(|p| !monkey_solutions.contains_key(&p.id)).collect_vec();

        for monkey in remaining_monkies {
            let possible_solution = monkey.try_get_solution(&monkey_solutions);
            if let Some(num) = possible_solution {
                monkey_solutions.insert(monkey.id.clone(), num);
            }
        }
    }

    return monkey_solutions["root"].clone();
    
}


#[derive(Debug, Clone)]
struct Monkey {
    id: String,
    job: MonkeyJob,
}

impl Monkey {
    fn parse(input: &str) -> IResult<&str, Self> { 
        let (input, id) = take_until(":")(input)?;

        let (input, monkey_job) = Monkey::parse_job(input)?;
        Ok((input, Monkey { id: id.to_string(), job: monkey_job}))
    }

    fn parse_job(input: &str) -> IResult<&str, MonkeyJob> { 
        alt((Monkey::parse_yell, Monkey::parse_operation))(input)
    }

    fn parse_yell(input: &str) -> IResult<&str, MonkeyJob> { 
        let (input, num) = (preceded(tag(": "), parse_numbers_i64))(input)?;
        Ok((input, MonkeyJob::Number(num)))
    }

    fn parse_operation(input: &str) -> IResult<&str, MonkeyJob> { 
        let id_length:usize = 4;
        let (input, (l, op, r)) = tuple((
            preceded(tag(": "), take(id_length)), 
            preceded(space1, one_of("*+/-")), 
            preceded(space1, take(id_length))))
            (input)?;

        let job = match op {
            '*' => MonkeyJob::Operation(l.to_string(), Operator::Multiply, r.to_string()),
            '+' => MonkeyJob::Operation(l.to_string(), Operator::Add, r.to_string()),
            '-' => MonkeyJob::Operation(l.to_string(), Operator::Subtract, r.to_string()),
            '/' => MonkeyJob::Operation(l.to_string(), Operator::Divide, r.to_string()),
            _ => unreachable!()

        };
        Ok((input, job))
    }

    fn try_get_solution(&self, monkey_solutions: &HashMap<String, i64>) -> Option<i64> {
        match &self.job {
            MonkeyJob::Operation(l, o, r) => {
                if !monkey_solutions.contains_key(l) || !monkey_solutions.contains_key(r) {
                    return Option::None;
                }
                Some(o.eval(monkey_solutions[l], monkey_solutions[r]))
            }
            MonkeyJob::Number(num) => Some(*num),
        }
    }
}


#[derive(Debug, Clone)]
enum MonkeyJob {
    Number(i64),
    Operation(String, Operator, String)
}
#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn eval(self, left: i64, right: i64) -> i64 {
        match self {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
            Operator::Multiply => left * right,
            Operator::Divide => left / right,
        }
    }
    fn solve_for_left(self, result: i64, right: i64) -> i64 {
        match self {
            Operator::Add => result - right,
            Operator::Subtract => result + right,
            Operator::Multiply => result / right,
            Operator::Divide => result * right,
        }
    }

    fn solve_for_right(self, result: i64, left: i64) -> i64 {
        match self {
            Operator::Add => result - left,
            Operator::Subtract => left - result,
            Operator::Multiply => result / left,
            Operator::Divide => left / result,
        }
    }
}
