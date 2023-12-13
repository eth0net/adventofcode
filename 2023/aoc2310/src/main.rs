use anyhow::Result;
use aoc2310::one;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let steps_1 = one::distance_to_end(INPUT);
    // let steps_2 = two::extrapolate(INPUT)?;

    println!("Distance 1: {steps_1}");
    // println!("Sum 2: {steps_2}");

    Ok(())
}
