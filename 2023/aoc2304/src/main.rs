use anyhow::{Context, Result};
use aoc2304::one;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let score = one::calculate_score(INPUT).with_context(|| "calculating score for input")?;

    println!("Total score: {}", score);

    Ok(())
}
