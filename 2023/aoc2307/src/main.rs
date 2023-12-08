use anyhow::Result;
use aoc2307::{one, two};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let winnings_one = one::winnings(INPUT)?;
    let winnings_two = two::winnings(INPUT)?;

    println!("Winnings 1: {winnings_one}");
    println!("Winnings 2: {winnings_two}");

    Ok(())
}
