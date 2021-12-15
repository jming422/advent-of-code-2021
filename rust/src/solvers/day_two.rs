use std::str::{FromStr, Lines};

use crate::error::{ErrorKind, MyError};

pub fn part_one(lines: Lines) -> Result<String, MyError> {
    let mut horiz = 0;
    let mut depth = 0;
    for line in lines {
        let mut words = line.split_whitespace();

        let direction = words.next().ok_or(MyError::new(ErrorKind::Other(
            "Not enough fields on one of the lines!".to_string(),
        )))?;
        let magnitude = words.next().ok_or(MyError::new(ErrorKind::Other(
            "Not enough fields on one of the lines!".to_string(),
        )))?;
        let delta = i32::from_str(magnitude).map_err(|e| MyError::wrap_parse_int(e))?;

        match direction {
            "forward" => horiz += delta,
            "up" => depth -= delta,
            "down" => depth += delta,
            _ => {
                return Err(MyError::new(ErrorKind::Other(format!(
                    "Invalid direction {}",
                    direction
                ))));
            }
        };
    }

    Ok(format!("{}", horiz * depth))
}

pub fn part_two(lines: Lines) -> Result<String, MyError> {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines {
        let mut words = line.split_whitespace();

        let direction = words.next().ok_or(MyError::new(ErrorKind::Other(
            "Not enough fields on one of the lines!".to_string(),
        )))?;
        let magnitude = words.next().ok_or(MyError::new(ErrorKind::Other(
            "Not enough fields on one of the lines!".to_string(),
        )))?;
        let delta = i32::from_str(magnitude).map_err(|e| MyError::wrap_parse_int(e))?;

        match direction {
            "forward" => {
                horiz += delta;
                depth += aim * delta;
            }
            "up" => aim -= delta,
            "down" => aim += delta,
            _ => {
                return Err(MyError::new(ErrorKind::Other(format!(
                    "Invalid direction {}",
                    direction
                ))));
            }
        };
    }

    Ok(format!("{}", horiz * depth))
}
