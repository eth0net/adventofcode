use anyhow::Result;
use aoc2308::{one, two};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let steps = one::map_steps(INPUT)?;
    let steps_two = two::map_steps(INPUT)?;
    println!("Map steps 1: {steps}");
    println!("Map steps 2: {steps_two}");
    Ok(())
}
