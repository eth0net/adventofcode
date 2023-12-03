use anyhow::{Context, Result};

use aoc2302::{one, two};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let possible = one::sum_possible(INPUT).with_context(|| "summing possible games")?;
    let power = two::sum_sets_power(INPUT).with_context(|| "summing sets powers")?;

    println!("Total: {possible}");
    println!("Power: {power}");

    Ok(())
}
