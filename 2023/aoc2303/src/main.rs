use anyhow::Result;

use aoc2303::{one, two};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let sum_parts = one::sum_parts(INPUT)?;
    let sum_gears = two::sum_gear_ratios(INPUT)?;

    println!("Sum of parts: {}", sum_parts);
    println!("Sum of gears: {}", sum_gears);

    Ok(())
}
