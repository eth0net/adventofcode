use anyhow::Result;
use aoc2307::one;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let winnings = one::winnings(INPUT)?;

    println!("Winnings: {winnings}");

    Ok(())
}
