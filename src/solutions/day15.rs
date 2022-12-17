use core::panic;
use std::cmp;
use num::{bigint, BigInt};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1},
    combinator::{map, map_res, recognize, opt},
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

pub fn part_two(input: String) -> BigInt {
    let sensors = get_sensors(input);
    let point = find_the_beacon(sensors, 4000000);

    calculate_frequency(point)
}

fn calculate_frequency(point: Point) -> BigInt {
    let big_x = bigint::BigInt::from(point.x);
    let frequency = big_x * bigint::BigInt::from(4000000) + bigint::BigInt::from(point.y);
    return frequency;
}

fn find_the_beacon(sensors: Vec<Sensor>, max_coord: i32) -> Point {
    for row in 0..=max_coord {
        let occupied_spaces = get_occupied_spaces_in_row_with_range(sensors.to_vec(), row, 0, max_coord);

        if occupied_spaces != max_coord {
            let x_location = find_x_location_of_beacon(sensors.to_vec(), row, 0, max_coord);
            return Point::new(x_location, row);
        }
    }
    panic!("Didn't find it");
}

fn get_sensors(input: String) -> Vec<Sensor> {
    input
    .lines()
    .map(|l| Sensor::parse(l.trim()).unwrap().1)
    .collect_vec()
}

fn get_occupied_spaces_in_row_with_range(sensors: Vec<Sensor>, row: i32, min: i32, max: i32) -> i32 {
    let unique_values = flatten_ranges(&sensors
        .iter()
        .map(|sensor| sensor.get_covered_range_in_row_with_range(row, min, max))
        .filter(|pair| pair.is_some())
        .map(|pair| pair.unwrap())
        .collect::<std::collections::BinaryHeap<_>>().into_sorted_vec());

        unique_values.iter().map(|(s, e)| 
        {
            e - s}
        ).sum()
}

fn find_x_location_of_beacon(sensors: Vec<Sensor>, row: i32, min: i32, max: i32) -> i32 { 
    let unique_values = flatten_ranges(&sensors
        .iter()
        .map(|sensor| sensor.get_covered_range_in_row_with_range(row, min, max))
        .filter(|pair| pair.is_some())
        .map(|pair| pair.unwrap())
        .collect::<std::collections::BinaryHeap<_>>().into_sorted_vec());


    if unique_values.len() == 2 { 
        let first = unique_values.get(0).unwrap();
        return first.1 + 1;
    }

    if unique_values.len() == 1 {
        let line = unique_values.get(0).unwrap();

        return match line.0.cmp(&min) {
            cmp::Ordering::Greater => min,
            _ => max
        }
    }

    panic!("Help");
}

fn get_occupied_spaces_in_row(sensors: Vec<Sensor>, row: i32) -> i32 {

    let unique_values = flatten_ranges(&sensors
        .iter()
        .map(|sensor| sensor.get_covered_range_in_row(row))
        .filter(|pair| pair.is_some())
        .map(|pair| pair.unwrap())
        .collect::<std::collections::BinaryHeap<_>>().into_sorted_vec());


    unique_values.iter().map(|(s, e)| 
    {
        e - s}
    ).sum()
}

// https://www.geeksforgeeks.org/merging-intervals/
fn flatten_ranges(ranges: &[(i32, i32)]) -> Vec<(i32, i32)> {
    if ranges.len() == 0 {
        return Vec::new()
    }
    (!ranges.is_empty()).then_some(ranges.iter()
        .skip(1)
        .fold(vec![ranges[0]], |flat, &r|
            std::iter::once(flat.len() - 1).fold(flat, |mut flat, last| {
                match flat[last].0 <= r.0 && r.0 <= flat[last].1 {
                    true => flat[last].1 = flat[last].1.max(r.1),
                    _ => flat.push(r),
                }

                flat
            })))
        .unwrap_or_default()
}


fn parse_numbers(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i32::from_str(s)
    })(input)?;

    Ok((i, number))
}

fn manhattan_distance(p1:Point, p2:Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

#[derive(Clone, Copy)]
struct Sensor {
    closest_beacon: Point,
    position: Point,
    distance: i32
}

impl Sensor {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Sensor at ")(input)?;
        let parse_two_points = separated_pair(Sensor::parse_sensor, tag(": closest beacon is at "), Sensor::parse_sensor);
        map(parse_two_points, |(p0, p1)| Sensor { position: p0, closest_beacon: p1, distance: manhattan_distance(p0, p1)})(input)
    }

    fn parse_sensor(input: &str) -> IResult<&str, Point> {
        let (input,_) = tag("x=")(input)?;
        let (input, (num1,_, num2)) = tuple((parse_numbers, tag(", y="), parse_numbers))(input)?;

        Ok((input, Point::new(num1, num2)))
    }

    fn get_covered_range_in_row(&self, y_row: i32) -> Option<(i32, i32)> {

        let x_radia = self.distance - (self.position.y - y_row).abs();
        if x_radia <= 0 {
            return Option::None
        }

        return Option::Some((self.position.x - x_radia, self.position.x + x_radia));
    }

    
    fn get_covered_range_in_row_with_range(&self, y_row: i32, min: i32, max: i32) -> Option<(i32, i32)> {

        let x_radia = self.distance - (self.position.y - y_row).abs();
        if x_radia <= 0 {
            return Option::None
        }

        return Option::Some((cmp::max(self.position.x - x_radia, min), cmp::min(self.position.x + x_radia, max)));
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
    #[test]
    fn sensor_can_calculate_its_coverage_distance() {
        let input = "Sensor at x=8, y=7: closest beacon is at x=2, y=10";

        
        let sensors = get_sensors(input.to_string());
        let result = sensors.get(0).unwrap().distance;

        assert_eq!(result, 9);
    }

    #[test]
    fn sensor_can_get_coverage_on_single_line() {
        let input = "Sensor at x=8, y=7: closest beacon is at x=2, y=10";

        
        let sensors = get_sensors(input.to_string());
        let result = get_occupied_spaces_in_row(sensors, 7);
        
        assert_eq!(result, 18);
    }

    #[test]
    fn sensor_can_get_no_coverage_for_far_line() {
        let input = "Sensor at x=8, y=7: closest beacon is at x=2, y=10";

        
        let sensors = get_sensors(input.to_string());
        let result = get_occupied_spaces_in_row(sensors, 2000);
        
        assert_eq!(result, 0);
    }
}