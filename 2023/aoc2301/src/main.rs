use anyhow::Result;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let first = aoc2301::one::calibrate(INPUT)?;
    let second = aoc2301::two::calibrate(INPUT)?;

    println!("Calibration value 1: {first}");
    println!("Calibration value 2: {second}");

    Ok(())
}
