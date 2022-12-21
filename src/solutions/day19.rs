use crate::tools;
use itertools::Itertools;
use nom::{bytes::complete::take_till, IResult};
use std::{
    fmt::{self, Display},
    ops::{Add, Sub, AddAssign, SubAssign},
    vec,
};

pub fn part_one(input: String) -> impl Display {
    let blueprints = parse_blueprints(input);

    let mut quality_sum = 0;        
    const MINUTES_TOTAL: i32 = 24;

    blueprints.iter().for_each(|bp| {

        let operation = Operation {
            clay_robots: 0,
            geode_robots: 0,
            obsidian_robots: 0,
            ore_robots: 1,
            next_model: None,
            supplies: Supplies { ore: 0, clay: 0, obsidian: 0, geodes: 0 }
        };
        let strat = optimize(
            &bp,
            MINUTES_TOTAL,
            operation,
            MINUTES_TOTAL,
        );

        println!("Blueprint Id: {}", bp.id);
        println!("  Geodes Cracked: {}", strat);
        quality_sum = quality_sum + (strat * bp.id);
    });

    quality_sum
}

pub fn part_two(input: String) -> impl Display {
    let blueprints = parse_blueprints(input);

    let mut quality_sum = 1;
    const MINUTES_TOTAL: i32 = 32;

    blueprints.iter().take(3).for_each(|bp| {
        let operation = Operation {
            clay_robots: 0,
            geode_robots: 0,
            obsidian_robots: 0,
            ore_robots: 1,
            next_model: None,
            supplies: Supplies { ore: 0, clay: 0, obsidian: 0, geodes: 0 }
        };
        let strat = optimize(
            &bp,
            MINUTES_TOTAL,
            operation,
            MINUTES_TOTAL,
        );

        println!("Strategy for Id {}", bp.id);
        println!("  Geodes Cracked: {}", strat);
        quality_sum = quality_sum * strat;
    });

    quality_sum
}

fn parse_blueprints(input: String) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| Blueprint::parse(line).unwrap().1)
        .collect_vec()
}

fn optimize(
    blueprint: &Blueprint,
    minutes_left: i32,
    mut operation: Operation,
    start_minutes: i32,
) -> i32 {
    
    let mut max_geode = 0;
    let models = vec![
        RobotModel::Geode,
        RobotModel::Obsidian,
        RobotModel::Ore,
        RobotModel::Clay,
    ];

    let mut minutes_left = minutes_left;
    let harvesting_robots = operation.clone();

    while operation.next_model.is_some() && minutes_left > 0 {
        minutes_left = minutes_left - 1;

        match operation.next_model {
            Some(model) => {
                if blueprint.can_afford(model, operation.supplies) {
                    match model {
                        RobotModel::Ore => operation.ore_robots = operation.ore_robots + 1,
                        RobotModel::Clay => operation.clay_robots = operation.clay_robots + 1,
                        RobotModel::Obsidian => operation.obsidian_robots = operation.obsidian_robots + 1,
                        RobotModel::Geode => operation.geode_robots = operation.geode_robots + 1,
                    }

                    operation.supplies -= blueprint.get_cost_for_model(model);
                    operation.next_model = None;
                }
            }
            None => (),
        }

        let gathered = Supplies::new(
            harvesting_robots.ore_robots,
            harvesting_robots.clay_robots,
            harvesting_robots.obsidian_robots,
            harvesting_robots.geode_robots,
        );
        
        operation.supplies += gathered;
    }
    
    max_geode = max_geode.max(operation.supplies.geodes);
    if minutes_left > 0 {
        for model in models.iter() {
            if minutes_left < 1 || (minutes_left < 4 && !matches!(model, RobotModel::Geode))
            {
                continue;
            }

            if(matches!(model, RobotModel::Geode) && operation.obsidian_robots == 0) {
                continue;
            }

            if(matches!(model, RobotModel::Obsidian) && operation.clay_robots == 0) {
                continue;
            }

            if matches!(model, RobotModel::Ore)
                && blueprint.max_ore_cost_per_turn <= operation.ore_robots
                || matches!(model, RobotModel::Clay)
                    && blueprint.max_clay_per_turn <= operation.clay_robots
                || matches!(model, RobotModel::Obsidian)
                    && blueprint.max_obsidian_per_turn <= operation.obsidian_robots
            {
                continue;
            }

            if (start_minutes - minutes_left) > 20 && matches!(model, RobotModel::Ore) {
                continue;
            }

            if (start_minutes - minutes_left) > 21 && matches!(model, RobotModel::Clay) {
                continue;
            }

            let mut next_opertation = operation.clone();
            next_opertation.next_model = Some(*model);

            let strat = optimize(
                blueprint,
                minutes_left,
                next_opertation,
                start_minutes,
            );

            max_geode = max_geode.max(strat);
        }
    }
    
    return max_geode;
}

#[derive(Debug, Hash, Clone, Copy)]
struct Operation {
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    next_model: Option<RobotModel>,
    supplies: Supplies
}

#[derive(Debug, Hash, Clone, Copy)]
struct Supplies {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
}
impl Add for Supplies {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geodes: self.geodes + other.geodes,
        }
    }
}


impl AddAssign for Supplies {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            geodes: self.geodes + other.geodes,
            obsidian: self.obsidian + other.obsidian
        };
    }
}

impl SubAssign for Supplies {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            geodes: self.geodes - other.geodes,
            obsidian: self.obsidian - other.obsidian
        };
    }
}

impl Sub for Supplies {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geodes: self.geodes - other.geodes,
        }
    }
}
impl Supplies {
    fn new(ore: i32, clay: i32, obsidian: i32, geodes: i32) -> Self {
        Supplies {
            ore,
            clay,
            obsidian,
            geodes,
        }
    }

    fn can_cover_cost(&self, cost: Supplies) -> bool {
        return self.ore >= cost.ore
            && self.clay >= cost.clay
            && self.obsidian >= cost.obsidian
            && self.geodes >= cost.geodes;
    }
}

impl fmt::Display for Supplies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ore: {}, Clay: {}, Obsidian: {}, Geodes: {}",
            self.ore, self.clay, self.obsidian, self.geodes
        )
    }
}

#[derive(Debug, Hash, Clone, Copy)]
enum RobotModel {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct Blueprint {
    id: i32,
    ore_robot_cost: Supplies,
    clay_robot_cost: Supplies,
    obsidian_robot_cost: Supplies,
    geode_robot_cost: Supplies,
    max_ore_cost_per_turn: i32,
    max_clay_per_turn: i32,
    max_obsidian_per_turn: i32,
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
        let max_ore_use_per_turn =
            ore_cost.max(clay_ore_cost.max(obsidian_ore_cost.max(geode_ore_cost)));
        let max_clay_per_turn = obsidian_clay_cost;
        let max_obsidian_per_turn = geode_obsidian_cost;

        let bp = Blueprint {
            id,
            ore_robot_cost: Supplies::new(ore_cost, 0, 0, 0),
            clay_robot_cost: Supplies::new(clay_ore_cost, 0, 0, 0),
            obsidian_robot_cost: Supplies::new(obsidian_ore_cost, obsidian_clay_cost, 0, 0),
            geode_robot_cost: Supplies::new(geode_ore_cost, 0, geode_obsidian_cost, 0),
            max_ore_cost_per_turn: max_ore_use_per_turn,
            max_clay_per_turn,
            max_obsidian_per_turn,
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
