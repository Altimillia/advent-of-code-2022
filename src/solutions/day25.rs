use std::{fmt::Display, collections::HashMap};

pub fn part_one(input: String) -> impl Display {
    let mut sum: i64 = 0;
    input.lines().for_each(|line| {
        sum += snafu_to_dec(line);
    });
    let snafu = dec_to_snafu(sum);
    snafu
}

pub fn part_two(input: String) -> impl Display {
    "I win"
}


fn snafu_to_dec(snafu: &str) -> i64 {
    let d: HashMap<char, i64> =
        HashMap::from([('=', -2), ('-', -1), ('0', 0), ('1', 1), ('2', 2)]);
    let mut n: i64 = 0;
    for (i, ch) in snafu.char_indices().rev() {
        let place: i64 = (snafu.len() - i - 1) as i64;
        n += d[&ch] * 5_i64.pow(place as u32);
    }
    n
}

fn dec_to_snafu(dec: i64) -> String {
    let mut snafu = String::new();
    let mut n = dec;
    let s: HashMap<i64, char> =
        HashMap::from([(3, '='), (4, '-'), (0, '0'), (1, '1'), (2, '2')]);
    while n != 0 {
        let rem: i64 = n % 5;
        let digit = s[&rem];
        snafu.push(digit);
        n = (n + 2) / 5;
    }
    snafu.chars().rev().collect::<String>()
}
