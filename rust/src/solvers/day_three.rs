use std::{num::ParseIntError, str::Lines};

use crate::error::{ErrorKind, MyError};

struct Counter {
    zeros: u32,
    ones: u32,
}
impl Counter {
    fn new() -> Self {
        Self { zeros: 0, ones: 0 }
    }

    fn mode(&self) -> u32 {
        if self.ones >= self.zeros {
            1
        } else {
            0
        }
    }

    fn mode_char(&self) -> char {
        if self.ones >= self.zeros {
            '1'
        } else {
            '0'
        }
    }
}

pub fn part_one(lines: Lines) -> Result<String, MyError> {
    let mut lines = lines.peekable();
    let mut init = vec![];
    if let Some(first_line) = lines.peek() {
        for _ in first_line.chars() {
            init.push(Counter::new());
        }
    } else {
        return Err(MyError::new(ErrorKind::BadInput));
    }

    let counts = lines.try_fold(init, |mut acc, line| {
        for (i, c) in line.chars().enumerate() {
            if i >= acc.len() {
                return Err(MyError::new(ErrorKind::BadInput));
            }

            match c {
                '0' => acc[i].zeros += 1,
                '1' => acc[i].ones += 1,
                _ => return Err(MyError::new(ErrorKind::BadInput)),
            }
        }
        Ok(acc)
    })?;

    let num_shifts = counts.len() - 1;
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for (pos, digit) in counts.iter().map(|c| c.mode()).enumerate() {
        gamma |= digit << (num_shifts - pos);
        epsilon |= (digit ^ 1) << (num_shifts - pos);
    }

    Ok(format!("{}", gamma * epsilon))
}

fn fold_count_digits(mut acc: Counter, next: char) -> Result<Counter, MyError> {
    match next {
        '0' => acc.zeros += 1,
        '1' => acc.ones += 1,
        _ => return Err(MyError::new(ErrorKind::BadInput)),
    };
    Ok(acc)
}

fn char_vec_to_u32(cs: Vec<char>) -> Result<u32, ParseIntError> {
    u32::from_str_radix(&cs.into_iter().collect::<String>(), 2)
}

pub fn part_two(lines: Lines) -> Result<String, MyError> {
    // I initially thought these Vec's could potentially be arrays, but they
    // cannot, because although the size of both dimensions is constant, it is
    // not known at compile time, and arrays must have a known length at compile
    // time.
    let mut o2_vals: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    let mut co2_vals = o2_vals.clone();
    let num_digits = o2_vals
        .first()
        .map(|cs| cs.len())
        .ok_or(MyError::new(ErrorKind::BadInput))?;
    if num_digits == 0 || o2_vals.iter().any(|cs| cs.len() != num_digits) {
        return Err(MyError::new(ErrorKind::BadInput));
    };

    for i in 0..num_digits {
        let o2s = o2_vals.len() > 1;
        let co2s = co2_vals.len() > 1;
        if !o2s && !co2s {
            break;
        };

        if o2s {
            let o2_mode = o2_vals
                .iter()
                .map(|cs| cs[i]) // Safe b/c of check for consistent line length
                .try_fold(Counter::new(), fold_count_digits)?
                .mode_char();
            o2_vals.retain(|cs| cs[i] == o2_mode);
        }

        if co2s {
            let co2_mode = co2_vals
                .iter()
                .map(|cs| cs[i]) // Safe b/c of check for consistent line length
                .try_fold(Counter::new(), fold_count_digits)?
                .mode_char();
            co2_vals.retain(|cs| cs[i] != co2_mode);
        }
    }

    let o2_rating =
        char_vec_to_u32(o2_vals.swap_remove(0)).map_err(|e| MyError::wrap_parse_int(e))?;
    let co2_rating =
        char_vec_to_u32(co2_vals.swap_remove(0)).map_err(|e| MyError::wrap_parse_int(e))?;

    Ok(format!("{}", o2_rating * co2_rating))
}
