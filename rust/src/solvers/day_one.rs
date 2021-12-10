use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve(input: String) -> Result<String, ParseIntError> {
    let mut count: usize = 0;
    let mut prev: Option<isize> = None;
    for s in input.split_whitespace() {
        let num = isize::from_str(s)?;
        if prev.map_or(false, |p| p < num) {
            count += 1;
        }
        prev = Some(num);
    }

    Ok(format!("{}", count))
}
