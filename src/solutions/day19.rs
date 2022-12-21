use std::{fmt::{Display, self}, ops::{Add, Sub}, vec};
use crate::tools;
use itertools::Itertools;
use nom::{bytes::complete::take_till, IResult};

pub fn part_one(input: String) -> impl Display {

    let blueprints = parse_blueprints(input);

    let mut quality_sum = 0;
    blueprints.iter().for_each(|bp| {
        println!("ID: {}", bp.id);
        const minutes_total:i32 = 24;
        let operation = Operation { clay_robots: 0, geode_robots: 0, obsidian_robots: 0, ore_robots: 1};
        let strat = optimize(&bp, Supplies::new(0,0,0,0), minutes_total, operation);

        println!("{}", bp.max_ore_cost_per_turn);
        println!("Strategy for Id {}", bp.id);
        println!("  {}", strat.supplies);
        quality_sum = quality_sum + (strat.supplies.geodes * bp.id);

        // strat.records.iter().for_each(|r| {

        //     match r {
        //         Record::Nothing(minutes, supplies) => println!("      Minute: {} Did nothing. Have {}", minutes_total - minutes, supplies),
        //         Record::BuiltRobot(minutes, supplies, model, _) => println!("      Minute: {} Built {:?}. Have {}",minutes_total - minutes, model, supplies),
        //     }
        // });
    });

    quality_sum
}

pub fn part_two(input: String) -> impl Display {
    0
}

fn parse_blueprints(input: String) -> Vec<Blueprint> {
    input.lines().map(|line| Blueprint::parse(line).unwrap().1).collect_vec()
}

fn optimize(blueprint: &Blueprint, supplies: Supplies, minutes_left: i32, operation: Operation) -> Strategy {

    let mut strategies:Vec<Strategy> = Vec::new();
    let total_supplies = supplies.clone();
    let actions = vec![Action::Nothing, Action::Build];

    let models = vec![
        RobotModel::Ore,
        RobotModel::Clay,
        RobotModel::Obsidian,
        RobotModel::Geode 
        ];

    if minutes_left > 0
    {
        // Gather Phase
        let remaining_minutes = minutes_left - 1;


        let gathered = Supplies::new(operation.ore_robots, operation.clay_robots, operation.obsidian_robots, operation.geode_robots);


        for action in actions {
            match action {
                Action::Nothing => {
                    let total_supplies = total_supplies + gathered;

                    //let mut branch = records.clone();
                    //branch.push(Record::Nothing(remaining_minutes, total_supplies));

                    let strat = optimize(blueprint, total_supplies, remaining_minutes, operation);
                    strategies.push(Strategy { 
                        supplies: strat.supplies
                        //, records: strat.records 
                    })
                },
                Action::Build => {
                    for model in models.iter() {
                        if !blueprint.can_afford(*model, total_supplies) 
                            || remaining_minutes < 1
                            || (remaining_minutes < 2 && !matches!(model, RobotModel::Geode)) {
                            continue;
                        }

                        if matches!(model, RobotModel::Ore) && blueprint.max_ore_cost_per_turn <= operation.ore_robots ||
                            matches!(model, RobotModel::Clay) && blueprint.obsidian_robot_cost.clay <= operation.clay_robots ||
                            matches!(model, RobotModel::Obsidian) && blueprint.geode_robot_cost.obsidian <= operation.obsidian_robots {
                            continue;
                        }

                        if remaining_minutes < 13 && matches!(model, RobotModel::Ore) {
                            continue;
                        }
                        if remaining_minutes < 9 && matches!(model, RobotModel::Clay) {
                            continue;
                        }

                        let mut branch = operation.clone();
                        let branched_supplies = total_supplies - blueprint.get_cost_for_model(*model) + gathered;

                        match model {
                            RobotModel::Ore => branch.ore_robots = branch.ore_robots + 1,
                            RobotModel::Clay => branch.clay_robots = branch.clay_robots + 1,
                            RobotModel::Obsidian => branch.obsidian_robots = branch.obsidian_robots + 1,
                            RobotModel::Geode => branch.geode_robots = branch.geode_robots + 1,
                        }

                        let strat = optimize(blueprint, branched_supplies, remaining_minutes, branch);

                        strategies.push(Strategy { 
                            supplies: strat.supplies, 
                            //records: strat.records 
                        });
                        if matches!(model, RobotModel::Geode) {
                            break;
                        }
                    }
                }
            }
        }
    }
    else {
        strategies.push(Strategy { supplies: total_supplies.clone()
            //, records: records.clone() 
        });
    }

    let mut best_strategy = Strategy { 
        supplies: Supplies::new(-1,-1,-1,-1)};
    for strategy in strategies {
        if strategy.supplies.geodes > best_strategy.supplies.geodes {
            best_strategy = strategy;
        }
    }

    //println!("Geodes {} on minute {}", best_strategy.supplies.geodes, minutes_left);
    return best_strategy;
}

struct Strategy {
    supplies: Supplies,
    // records: Vec<Record>
}
#[derive(Debug, Hash, Clone, Copy)]
enum Action {
    Nothing,
    Build
}

#[derive(Debug, Hash, Clone, Copy)]
enum Record {
    Nothing(i32, Supplies),
    BuiltRobot(i32, Supplies, RobotModel, Supplies)
}

#[derive(Debug, Hash, Clone, Copy)]
struct Operation {
    ore_robots: i32,    
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

#[derive(Debug, Hash, Clone, Copy)]
struct Supplies {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32
}
impl Add for Supplies {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { ore: self.ore + other.ore, clay: self.clay + other.clay, obsidian: self.obsidian + other.obsidian, geodes: self.geodes + other.geodes }
    }
}

impl Sub for Supplies {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { ore: self.ore - other.ore, clay: self.clay - other.clay, obsidian: self.obsidian - other.obsidian, geodes: self.geodes - other.geodes }
    }
}
impl Supplies {
    fn new(ore: i32, clay: i32, obsidian: i32, geodes: i32) -> Self {
        Supplies { ore, clay, obsidian, geodes }
    }

    fn can_cover_cost(&self, cost: Supplies) -> bool {
        return self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian && self.geodes >= cost.geodes
    }
} 


impl fmt::Display for Supplies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ore: {}, Clay: {}, Obsidian: {}, Geodes: {}", self.ore, self.clay, self.obsidian, self.geodes)
    }
}

#[derive(Debug, Hash, Clone, Copy)]
enum RobotModel {
    Ore,
    Clay,
    Obsidian,
    Geode
}

#[derive(Debug, Hash, Clone, Copy)]
struct Robot {
    gathering_speed: i32,
    model: RobotModel
}

impl Robot {
    fn new(model:RobotModel) -> Self {
        Robot { gathering_speed: 1, model: model }
    }

    fn gather(&self) -> Supplies {
        match self.model {
            RobotModel::Ore => Supplies::new(self.gathering_speed, 0, 0, 0),
            RobotModel::Clay => Supplies::new(0, self.gathering_speed, 0, 0),
            RobotModel::Obsidian => Supplies::new(0, 0, self.gathering_speed, 0),
            RobotModel::Geode => Supplies::new(0, 0, 0, self.gathering_speed),
        }
    }
}
// #[derive(Debug)]
// struct ClayRobot {
//     gathering_speed: i32
// }

// impl Robot for ClayRobot {
//     fn gather(&self, minutes_spent: i32) -> Supplies {
//         Supplies::new(0, minutes_spent * self.gathering_speed, 0, 0)
//     }
// }

// #[derive(Debug)]
// struct ObsidianRobot {
//     gathering_speed: i32
// }

// impl Robot for ObsidianRobot {
//     fn gather(&self, minutes_spent: i32) -> Supplies {
//         Supplies::new(0, 0, self.gathering_speed * minutes_spent, 0)
//     }
// }

// #[derive(Debug)]
// struct GeodeRobot {
//     gathering_speed: i32
// }

// impl Robot for GeodeRobot {
//     fn gather(&self, minutes_spent: i32) -> Supplies {
//         Supplies::new(0, 0, 0, self.gathering_speed * minutes_spent)
//     }
// }
// #[derive(Debug)]
// struct OreRobot {
//     gathering_speed: i32
// }

// impl Robot for OreRobot {
//     fn gather(&self, minutes_spent: i32) -> Supplies {
//         Supplies::new(minutes_spent * self.gathering_speed, 0, 0, 0)
//     }
// }

// trait Robot : std::fmt::Debug { 
//     fn gather(&self, minutes_spent: i32) -> Supplies;
// }

struct Blueprint {
    id: i32,
    ore_robot_cost: Supplies,
    clay_robot_cost: Supplies,
    obsidian_robot_cost: Supplies,
    geode_robot_cost: Supplies,
    max_ore_cost_per_turn: i32
}
impl Blueprint {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, id) = Blueprint::get_next_number(input)?;
        let (input, ore_cost) = Blueprint::get_next_number(input)?;
        let (input, clay_ore_cost) = Blueprint::get_next_number(input)?;
        let (input, obsidian_ore_cost) = Blueprint::get_next_number(input)?;
        let (input, obsidian_clay_cost) = Blueprint::get_next_number(input)?;
        let (input, geode_ore_cost) = Blueprint::get_next_number(input)?;
        let (input, geode_obsidian_cost) = Blueprint::get_next_number(input)?;
        let max_ore_use_per_turn = ore_cost.max( clay_ore_cost.max( obsidian_ore_cost.max( geode_ore_cost)));

        let bp = Blueprint { id, 
            ore_robot_cost: Supplies::new(ore_cost,0,0,0), 
            clay_robot_cost: Supplies::new(clay_ore_cost,0,0,0),
            obsidian_robot_cost: Supplies::new(obsidian_ore_cost, obsidian_clay_cost,0,0),
            geode_robot_cost: Supplies::new(geode_ore_cost,0, geode_obsidian_cost,0),
            max_ore_cost_per_turn: max_ore_use_per_turn
        };

        return Ok((input, bp));
    }

    fn can_afford(&self, model: RobotModel, current_supplies: Supplies) -> bool {
        match model {
            RobotModel::Ore => current_supplies.can_cover_cost(self.ore_robot_cost),
            RobotModel::Clay => current_supplies.can_cover_cost(self.clay_robot_cost),
            RobotModel::Obsidian => current_supplies.can_cover_cost(self.obsidian_robot_cost),
            RobotModel::Geode => current_supplies.can_cover_cost(self.geode_robot_cost),
        }
    }

    fn get_cost_for_model(&self, model: RobotModel) -> Supplies {
        match model {
            RobotModel::Ore => self.ore_robot_cost,
            RobotModel::Clay => self.clay_robot_cost,
            RobotModel::Obsidian => self.obsidian_robot_cost,
            RobotModel::Geode => self.geode_robot_cost,
        }
    }

    fn get_next_number(input: &str) -> IResult<&str, i32> {
        let (input, _) = (take_till(|c| tools::is_digit(c)))(input)?;
        (tools::parse_numbers)(input)
    }
}