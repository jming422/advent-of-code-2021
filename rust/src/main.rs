#![allow(dead_code)]
use std::{error::Error, fs, io};

mod solvers;
use crate::solvers::day_one;

fn load_input(i: u8) -> io::Result<String> {
    fs::read_to_string(format!("../puzzles/{}-input.txt", i))
}

static PUZZLE: u8 = 1;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_input(PUZZLE)?;
    let output = day_one::part_two(input)?;

    println!("{}", output);

    Ok(())
}
