use anyhow::{Context, Result};
use aoc2304::{one, two};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let score = one::calculate_score(INPUT).with_context(|| "calculating score for input")?;
    let count = two::total_scratchcards(INPUT).with_context(|| "counting total scratchcards")?;

    println!("Total score:  {}", score);
    println!("Scratchcards: {}", count);

    Ok(())
}
