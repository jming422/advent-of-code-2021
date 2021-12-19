#![allow(dead_code)]
use std::{error::Error, fs, io};

mod error;
mod solvers;

fn load_input(i: u8) -> io::Result<String> {
    fs::read_to_string(format!("../puzzles/{}-input.txt", i))
}

static PUZZLE: u8 = 4;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_input(PUZZLE)?;
    let output = solvers::day_four::part_two(input.lines())?;

    println!("{}", output);

    Ok(())
}
