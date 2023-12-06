use anyhow::{Context, Result};
use aoc2306::{one, two};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let sequence = one::race_result(INPUT).with_context(|| "part one")?;
    let single = two::race_result(INPUT).with_context(|| "part two")?;

    println!("Sequence: {sequence}");
    println!("Single: {single}");

    Ok(())
}
