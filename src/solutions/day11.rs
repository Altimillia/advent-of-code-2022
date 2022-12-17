use std::{error};
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


pub fn part_one(input: String) -> u64 {

    let mut monkeys:Vec<Monkey> = Vec::new();
    input.split("\n\n").for_each(|monkey_def| {
        monkeys.push(Monkey::new(monkey_def).unwrap());
    });

    let mut keep_away = KeepAway { monkeys: monkeys };
    let divisor_product = keep_away.monkeys.iter().map(|m| m.evaluate_number).product::<u64>();
    for _ in 0..20 {
        keep_away.run_round(divisor_product);
    }


    let mut sorted_monkeys = keep_away.monkeys.to_vec();
    sorted_monkeys.sort_by(|a,b| b.items_inspected.cmp(&a.items_inspected));
    return sorted_monkeys[0].items_inspected * sorted_monkeys[1].items_inspected;
}

pub fn part_two(input: String) -> u64 {
    let mut monkeys:Vec<Monkey> = Vec::new();
    input.split("\n\n").for_each(|monkey_def| {
        monkeys.push(Monkey::new(monkey_def).unwrap());
    });

    let mut keep_away = KeepAway { monkeys: monkeys };
    let divisor_product = keep_away.monkeys.iter().map(|m| m.evaluate_number).product::<u64>();

    for _ in 0..10000 {
        keep_away.run_round(divisor_product);
    }

    let mut sorted_monkeys = keep_away.monkeys.to_vec();
    sorted_monkeys.sort_by(|a,b| b.items_inspected.cmp(&a.items_inspected));
    return sorted_monkeys[0].items_inspected * sorted_monkeys[1].items_inspected;
}


struct KeepAway {
    monkeys: Vec<Monkey>
}

impl KeepAway {

    fn run_round(&mut self, divisor_product: u64) {
        let num_monkeys = self.monkeys.len();

        for i in 0..num_monkeys {
            let monkey_copy;
            {
                let monkey = &mut self.monkeys[i];
                monkey_copy = monkey.clone();
                monkey.items_inspected += monkey_copy.items.len() as u64;
            }

            for mut item in monkey_copy.items.iter().copied() {
                item = Item { worry_level: item.worry_level % divisor_product };
                item = monkey_copy.inspect(item);

                let toss_to = monkey_copy.decide(item);

                self.monkeys.get_mut(toss_to as usize).unwrap().catch_mut(item);
            }
            self.monkeys[i].items.clear();
        }
    }


}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    id: i32,
    operation: meval::Expr,
    evaluate_number: u64,
    evaluate_true: i32,
    evaluate_false: i32,
    items_inspected: u64,
}

impl Monkey {
    fn new(monkey_def: &str) -> Result<Monkey> {

        fn id_parser(line: &str) -> Result<MonkeyId> {

            let line = line.replace(":", "");
            return Ok(MonkeyId {value: i32::from_str_radix(line.split_whitespace().nth(1).unwrap(), 10)? });
        }

        fn items_parser(line: &str) -> Result<Vec<Item>> {
            return Ok(line.split(":").nth(1).unwrap().split(",").map(|item| {
                return Item { worry_level: u64::from_str_radix(item.trim(), 10).unwrap() };
            }).collect());
        }

        fn operation_parser(line: &str) -> Result<meval::Expr> {
            return Ok(line.split("=").nth(1).unwrap().to_string().parse().unwrap());
        }

        fn get_last_number_in_line(line: &str) -> Result<i32> {
            return Ok(i32::from_str_radix(line.split_whitespace().last().unwrap(), 10).unwrap());
        }

        fn get_last_number_in_line_64(line: &str) -> Result<u64> {
            return Ok(u64::from_str_radix(line.split_whitespace().last().unwrap(), 10).unwrap());
        }
        let mut lines = monkey_def.lines().into_iter();

        let id = id_parser(lines.nth(0).unwrap());
        let items = items_parser(lines.nth(0).unwrap());
        let operation = operation_parser(lines.nth(0).unwrap());
        let eval_number = get_last_number_in_line_64(lines.nth(0).unwrap()).unwrap();
        let eval_true = MonkeyId { value: get_last_number_in_line(lines.nth(0).unwrap()).unwrap() };
        let eval_false = MonkeyId { value: get_last_number_in_line(lines.nth(0).unwrap()).unwrap() };

        return Ok(Monkey { id: id?.value, items: items?, operation: operation?, evaluate_number: eval_number, evaluate_true:eval_true.value, evaluate_false: eval_false.value, items_inspected: 0 });
    }

    fn inspect(&self, item: Item) -> Item {
        let op = self.operation.to_owned();
        let func = op.bind("old").unwrap();
        return Item { worry_level: func(item.worry_level as f64).round() as u64 };
    }

    fn decide(&self, item: Item) -> i32 {
        if item.worry_level % self.evaluate_number as u64 == 0 {
            return self.evaluate_true;
        }
        else {
            return self.evaluate_false
        }
    }


    fn catch_mut(&mut self, item: Item) {
        self.items.push(item);
    }

}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct MonkeyId {
    value: i32
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Item {
    worry_level: u64
}

#[derive(Debug, Clone)]
struct MathOperation {
    operation: meval::Expr
}

impl Operation for MathOperation {
    fn get_worry_level(&self, old: i32) -> i32 {
        let op = self.operation.to_owned();
        let func = op.bind("old").unwrap();
        return func(old as f64).round() as i32;
    }
}

trait Operation: std::fmt::Debug {
    fn get_worry_level(&self, old: i32) -> i32;
}

#[derive(Debug)]
struct DivisibleTest {
    division_number: i32
}

impl Test for DivisibleTest {
    fn evaluate(&self, input: i32) -> bool {
        return input % self.division_number == 0
    }
    
}

trait Test: std::fmt::Debug {
    fn evaluate(&self, input: i32) -> bool;
}