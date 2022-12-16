use std::{fmt::Display, ops::Range};

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

pub fn part_one(input: String) -> i32 {
    let sensors = get_sensors(input);
    let value = get_occupied_spaces_in_row(sensors, 2000000);
    value
}

fn get_sensors(input: String) -> Vec<Sensor> {
    input
    .lines()
    .map(|l| Sensor::parse(l.trim()).unwrap().1)
    .collect_vec()
}

fn get_occupied_spaces_in_row(sensors: Vec<Sensor>, row: i32) -> i32 {
    let unique_values = sensors
    .iter()
    .map(|sensor| sensor.get_covered_range_in_row(10))
    .map(|range| range.0..range.1)
    .flatten()
    .unique().collect_vec();

    unique_values.len() as i32
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


        let mut rt_value:Vec<Point> = Vec::new();
        let mut check_now:Vec<Point> = Vec::new();

        check_now.push(self.position);

        rt_value.push(self.position);

        while check_now.len() > 0 || !rt_value.contains(&self.closest_beacon) {
            let p = check_now.remove(0);

            for next in p.get_neighbors() {
                check_now.push(next);
                rt_value.push(next);
                println!("{}",next);
            }
        }

        rt_value

    }

    fn get_manhattan_distance(&self) -> i32 {
        return (self.closest_beacon.x - self.position.x).abs() + (self.closest_beacon.y - self.position.y).abs()
    }

    fn get_covered_range_in_row(&self, y_row: i32) -> (i32, i32) {
        let manhattan_distance = self.get_manhattan_distance();
        let x_radia = manhattan_distance - (self.position.y - y_row).abs();
        if x_radia <= 0 {
            return (0,0);
        }
        println!("{}", x_radia);

        return (self.position.x - x_radia, self.position.x + x_radia);
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day15::{get_sensors, get_occupied_spaces_in_row};

    use super::part_one;

    #[test]
    fn part_one_gets_value() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        
        let sensors = get_sensors(input.to_string());
        let result = get_occupied_spaces_in_row(sensors, 10);

        assert_eq!(result, 26);
    } 
}