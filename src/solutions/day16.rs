use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::{fmt::Display, collections::HashMap};
use std::str::FromStr;
use itertools::Itertools;
use nom::{bytes::complete::{tag, take}, IResult, combinator::{recognize, map_res, opt}, sequence::{preceded}, character::complete::{digit1}};
type DistanceMatrix<'a> = HashMap<String, HashMap<String, i32>>;

pub fn part_one(input: String) -> impl Display {
    let valves = get_valves(input);
    let distance_matrix = get_valve_distances(&valves);
    let to_open = valves.iter().filter(|v| v.flow_rate > 0).map(|v| v.id.clone()).collect_vec();
    recursive_path_finding(&valves, &to_open, &distance_matrix, "AA", 30, &vec!["AA".to_string()], &Vec::new()).flow
} 

pub fn part_two(input: String) -> impl Display {
    let valves = get_valves(input);
    let distance_matrix = get_valve_distances(&valves);
    let to_open = valves.iter().filter(|v| v.flow_rate > 0).map(|v| v.id.clone()).collect_vec();
    let PathFit { path: no_overlap, flow } = recursive_path_finding(&valves, &to_open, &distance_matrix, "AA", 26, &vec!["AA".to_string()], &Vec::new());
    
    let PathFit { path: _, flow: elephant_helping } = recursive_path_finding(&valves, &to_open, &distance_matrix, "AA", 26, &vec!["AA".to_string()], &no_overlap);
    flow + elephant_helping
} 


fn recursive_path_finding<'a>(
    valves: &Vec<Valve>,
    to_open: &Vec<String>,
    distance_matrix: &'a DistanceMatrix,
    start_valve: &str,
    minutes_left: i32,
    path: &Vec<String>,
    overlap: &Vec<String>
) -> PathFit {
    let mut paths: Vec<PathFit> = Vec::new();

    for valve in to_open {
        if overlap.contains(valve) {
            continue;
        }
        let distance = distance_matrix[valve][start_valve];

        let valve_ref = valves.iter().find(|v| v.id == *valve).unwrap();

        if distance >= minutes_left {

            // Not a candidate
            continue;
        }

        let minutes_after_operation = minutes_left - distance - 1;
        let flow_acheived = valve_ref.flow_rate * minutes_after_operation;

        let next_to_open:&Vec<String> = &to_open.iter().filter(|v| *v != valve).map(|v| v.clone()).collect_vec();
        let mut branch = path.clone();
        branch.push(valve.clone());
        let full_path = recursive_path_finding(valves, next_to_open, distance_matrix, valve, minutes_after_operation, &branch, &overlap);

        let mut add_path = path.clone();
        add_path.extend(full_path.path);

        paths.push(PathFit { path: add_path, flow: full_path.flow + flow_acheived });
    }

    let mut best_path = PathFit { path: Vec::new(), flow: 0 };
    for path_fit in paths {
        if path_fit.flow > best_path.flow {
            best_path = path_fit;
        }
    }
    best_path
}


fn get_valve_distances(valves: &Vec<Valve>) -> DistanceMatrix {
    let mut distances = HashMap::new();

    for start in valves {
        let start = start.clone();
        let distances_from = distances.entry(start.id.clone())
            .or_insert(HashMap::new());
            let mut to_visit = BinaryHeap::new();
            let mut visited = HashSet::new();
    
            to_visit.push(Visit {  id: start.id, distance: 0 });
    
            while let Some(visit) = to_visit.pop() {
                if !visited.insert(visit.id.clone()) {
                    continue;
                }
    
                let neighbours = &valves.iter().find(|v| v.id == visit.id.clone()).unwrap().leads_to;
                    for neighbour in neighbours {
                        let neighbour_clone = valves.iter().find(|v| v.id == *neighbour).unwrap().clone();
                        let new_dist = visit.distance + 1;
                        let use_dist = distances_from.get(&neighbour_clone.id)
                            .map_or(true, |&current_dist| {
                                current_dist > new_dist
                            });
                        
                        if use_dist {
                            distances_from.insert(neighbour_clone.id.clone(), new_dist);
                            to_visit.push(Visit { id: neighbour_clone.id, distance: new_dist });
                        }
                    }
                
            }
        }
        distances
       

    }


fn get_valves(input: String) -> Vec<Valve> {
    input
    .lines()
    .map(|l| Valve::parse(l.trim()).unwrap().1)
    .collect_vec()
}

fn parse_numbers(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i32::from_str(s)
    })(input)?;

    Ok((i, number))
}

#[derive(Debug, Clone)]
struct PathFit {
    path: Vec<String>,
    flow: i32,
}


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Valve {
    id: String,
    flow_rate: i32,
    leads_to: Vec<String>
}


#[derive(Debug)]
struct Visit {
    id: String,
    distance: i32
}

impl<'a> Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<'a> PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}
impl<'a> Eq for Visit {}

impl Valve {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Valve ")(input)?;

        let (input, id) = take(2 as usize)(input)?;
        let (input, _) = tag(" has flow rate=")(input)?;
        let (input, flow_rate) = (parse_numbers)(input)?;
        let (input, _) = (Valve::parse_out_plurals)(input)?;

        let (input, leads_to) = (Valve::parse_leads_to_valves)(input)?;
        Ok((input, Valve { id: id.to_string(), flow_rate: flow_rate, leads_to: leads_to.into_iter().map(|v| v.to_string() ).collect() }))
    }

    fn parse_out_plurals(input: &str) -> IResult<&str, &str> {
        let (input, r) = opt(tag("; tunnel leads to valve "))(input)?;
        if r.is_some() {
            return Ok((input, r.unwrap()));
        }

        let (input, plural) = opt(tag("; tunnels lead to valves "))(input)?;
        return Ok((input, plural.unwrap()));
    }

    fn parse_leads_to_valves(input: &str) -> IResult<&str, Vec<&str>> {
    
        if !input.contains(",") {
            return Ok((input, vec![input.trim()]));
        }
        let splits = input.split(",");
        let end_result = splits.map(|f| { 
            f.trim()
        }).collect_vec();

    
        Ok((input, end_result))
    }

}