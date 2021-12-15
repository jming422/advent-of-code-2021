use std::num::ParseIntError;
use std::str::{FromStr, Lines};

pub fn part_one(lines: Lines) -> Result<String, ParseIntError> {
    let mut count: usize = 0;
    let mut prev: Option<isize> = None;
    for s in lines {
        let num = isize::from_str(s)?;
        if prev.map_or(false, |p| p < num) {
            count += 1;
        }
        prev = Some(num);
    }

    Ok(format!("{}", count))
}

pub fn part_two(lines: Lines) -> Result<String, ParseIntError> {
    let mut nums = vec![];
    for s in lines {
        nums.push(isize::from_str(s)?);
    }

    let mut count: usize = 0;
    let mut prev: Option<isize> = None;
    for window in nums.windows(3) {
        let val = window.iter().sum();
        if prev.map_or(false, |p| p < val) {
            count += 1;
        }
        prev = Some(val);
    }

    Ok(format!("{}", count))
}
