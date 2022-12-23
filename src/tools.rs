use nom::{IResult, combinator::{map_res, recognize, opt}, sequence::preceded, character::complete::digit1, bytes::complete::tag};
use std::str::FromStr;

pub fn parse_numbers(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i32::from_str(s)
    })(input)?;

    Ok((i, number))
}

pub fn parse_numbers_f64(input: &str) -> IResult<&str, f64> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        f64::from_str(s)
    })(input)?;

    Ok((i, number))
}
pub fn parse_numbers_i64(input: &str) -> IResult<&str, i64> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i64::from_str(s)
    })(input)?;

    Ok((i, number))
}
pub fn parse_numbers_i128(input: &str) -> IResult<&str, i128> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i128::from_str(s)
    })(input)?;

    Ok((i, number))
}




pub fn is_digit(c: char) -> bool {
    c.is_digit(10)
  }