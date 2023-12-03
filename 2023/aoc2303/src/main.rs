use anyhow::Result;

use aoc2303::one;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let sum = one::sum_missing_parts(INPUT)?;

    println!("Sum of missing parts: {}", sum);

    Ok(())
}
