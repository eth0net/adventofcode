use anyhow::Result;
use aoc2308::one;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let steps = one::map_steps(INPUT)?;
    println!("Map steps: {steps}");
    Ok(())
}
