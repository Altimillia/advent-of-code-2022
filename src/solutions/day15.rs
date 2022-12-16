use std::fmt::Display;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, recognize, opt},
    multi::separated_list1,
    sequence::{separated_pair, tuple, preceded},
    IResult,
};
use std::str::FromStr;
use crate::domain::point::Point;

pub fn part_one(input: String) -> impl Display {
    let sensors = input
        .lines()
        .map(|l| Sensor::parse(l).unwrap().1)
        .collect_vec();


    for sensor in sensors {
        println!("{} {}", sensor.position, sensor.closest_beacon);
    }

    0
}

pub fn part_two(input: String) -> impl Display {
    0
}
fn parse_numbers(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i32::from_str(s)
    })(input)?;

    Ok((i, number))
}
struct Sensor {
    closest_beacon: Point,
    position: Point
}

impl Sensor {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Sensor at ")(input)?;
        let parse_two_points = separated_pair(Sensor::parse_sensor, tag(": closest beacon is at "), Sensor::parse_sensor);
        map(parse_two_points, |(p0, p1)| Sensor { position: p0, closest_beacon: p1})(input)
    }

    fn parse_sensor(input: &str) -> IResult<&str, Point> {
        let (input,_) = tag("x=")(input)?;
        let (input, (num1,_, num2)) = tuple((parse_numbers, tag(", y="), parse_numbers))(input)?;

        Ok((input, Point::new(num1, num2)))
    }

    fn get_covered_positions(&self) -> Vec<Point> {
        todo!()
    }
}