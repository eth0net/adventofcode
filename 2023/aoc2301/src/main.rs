use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let mut sum: usize = 0;

    for line in INPUT.lines() {
        let first = first_digit(line).with_context(|| "no first digit found")?;
        let last = last_digit(line).with_context(|| "no first digit found")?;

        let value = format!("{first}{last}")
            .parse::<usize>()
            .with_context(|| "parsing calibration value")?;

        sum += value;
    }

    println!("Calibration value: {sum}");

    Ok(())
}

fn first_digit(line: &str) -> Option<char> {
    line.chars().find(|c| c.is_ascii_digit())
}

fn last_digit(line: &str) -> Option<char> {
    line.chars().rev().find(|c| c.is_ascii_digit())
}
