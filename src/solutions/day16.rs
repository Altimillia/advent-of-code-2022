use std::fmt::Display;
use std::str::FromStr;
use itertools::Itertools;
use nom::{bytes::complete::{tag, take_while, take_while_m_n, take}, IResult, combinator::{recognize, map_res, opt, map}, sequence::{preceded, tuple}, character::complete::{char,digit1}, multi::separated_list1};

pub fn part_one(input: String) -> impl Display {
    let valves = get_valves(input);

    
    valves.iter().for_each(|v| {
        println!("Valve {}", v.id);
        v.leads_to.iter().for_each(|l| println!("{}", l))
    });
    valves.len()
} 

pub fn part_two(input: String) -> impl Display {
    0
} 


fn get_valves(input: String) -> Vec<Valve> {
    input
    .lines()
    .map(|l| Valve::parse(l.trim()).unwrap().1)
    .collect_vec()
}

fn parse_numbers(input: &str) -> IResult<&str, usize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        usize::from_str(s)
    })(input)?;

    Ok((i, number))
}


struct Valve {
    id: String,
    flow_rate: usize,
    leads_to: Vec<String>
}

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