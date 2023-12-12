use anyhow::Result;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let steps = one::extrapolate_sum(INPUT)?;

    println!("Sum 1: {steps}");

    Ok(())
}

mod one;
