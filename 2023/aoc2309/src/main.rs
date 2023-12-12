use anyhow::Result;
use aoc2309::{one, two};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let steps_1 = one::extrapolate(INPUT)?;
    let steps_2 = two::extrapolate(INPUT)?;

    println!("Sum 1: {steps_1}");
    println!("Sum 2: {steps_2}");

    Ok(())
}
