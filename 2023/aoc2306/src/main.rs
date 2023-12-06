use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let win = aoc2306::one::race_result(INPUT).with_context(|| "part one")?;

    println!("Result: {win}");

    Ok(())
}
