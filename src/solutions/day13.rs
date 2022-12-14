use serde_json::{json, Value};
use itertools::Itertools;
use std::{iter::Peekable, fs::read, cmp::Ordering};

pub fn part_one(input: String) -> usize { 

    let packets = read_input(input.as_str());
    let pairs = packets.chunks(2).map(|f| f.to_vec()).collect_vec();

    let pairs = pairs
        .iter()
        .map(|pair| compare_packets(&pair[0], &pair[1]))
        .enumerate()
        .filter(|(_, p)| p.is_some() && matches!(p.unwrap(), Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    return pairs;
}

pub fn part_two(input: String) -> usize {
    let mut packets = read_input(input.as_str());

    packets.extend([json!([[2]]), json!([[6]])]);
    packets.sort_by(|left,right| compare_packets(left, right).unwrap());

    let divider_packet_1 = packets.iter().position(|p| *p == json!([[2]])).unwrap() + 1;
    let divider_packet_2 = packets.iter().position(|p| *p == json!([[6]])).unwrap() + 1;

    return divider_packet_1 * divider_packet_2;
}

fn compare_packets(left: &Value, right: &Value) -> Option<Ordering> { 
    match(left, right) {
        (Value::Number(left), Value::Number(right)) => match left.as_u64().cmp(&right.as_u64()) {
            Ordering::Equal => None,
            order => Some(order)
        },
        (Value::Array(left), Value::Array(right)) => {
            if left.is_empty() || right.is_empty() {
                match left.len().cmp(&right.len()) {
                    Ordering::Equal => None,
                    order => Some(order),
                }
            } else if let Some(v) = compare_packets(&left[0], &right[0]) {
                Some(v)
            } else {
                compare_packets(&json!(left[1..]), &json!(right[1..]))
            }
        },
        (Value::Number(left), Value::Array(right)) => compare_packets(&json!(vec![left]), &json!(right)),
        (Value::Array(left), Value::Number(right)) => compare_packets(&json!(left), &json!(vec![right])),
        _ => Some(Ordering::Greater)
    }
}


fn read_input(input: &str) -> Vec<Value> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

// enum Term {
//     Int(usize),
//     List(Vec<Term>)
// }

// impl Term {
//     fn parse(line: &str) -> Option<Term> {
//         Self::parse_recursive(&mut line.chars().peekable())
//     }

//     fn parse_recursive(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Term> {
//         match chars.next() {
//             Some(c) => {

//             },
//             Some('[') => {

//             },
//             _ => None
//         }
//         return None;
//     }
// }