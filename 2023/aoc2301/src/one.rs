use anyhow::{Context, Result};

pub fn calibrate(input: &str) -> Result<u32> {
    input.lines().map(evaluate_line).sum()
}

fn evaluate_line(line: &str) -> Result<u32> {
    let first = first_digit(line).with_context(|| "no first digit found")?;
    let last = last_digit(line).with_context(|| "no first digit found")?;

    let value = format!("{first}{last}")
        .parse::<u32>()
        .with_context(|| "parsing calibration value")?;

    Ok(value)
}

fn first_digit(line: &str) -> Option<char> {
    line.chars().find(|c| c.is_ascii_digit())
}

fn last_digit(line: &str) -> Option<char> {
    line.chars().rev().find(|c| c.is_ascii_digit())
}
