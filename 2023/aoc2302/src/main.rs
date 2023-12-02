use anyhow::{Context, Result};
use aoc2302::one;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let sum = one::sum_possible(INPUT).with_context(|| "summing possible games")?;

    println!("Total: {sum}");

    Ok(())
}
