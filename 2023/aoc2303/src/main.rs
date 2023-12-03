use anyhow::Result;

use aoc2303::one;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let sum = one::sum_parts(INPUT)?;

    println!("Sum of parts: {}", sum);

    Ok(())
}
