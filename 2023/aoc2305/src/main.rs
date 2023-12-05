use anyhow::{Context, Result};
use aoc2305::one;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let closest = one::closest_seed_location(INPUT).with_context(|| "getting closest location")?;

    println!("Closest: {closest}");

    Ok(())
}
