use std::num::ParseIntError;

use anyhow::{bail, Context, Error, Result};

fn parse_isize_vec(s: &str) -> Result<Vec<isize>, Error> {
    s.split_ascii_whitespace()
        .map(str::parse::<isize>)
        .collect::<Result<Vec<isize>, ParseIntError>>()
        .with_context(|| "parsing isize vec from string")
}

pub fn extrapolate_next(s: &str) -> Result<isize> {
    let mut sequence = parse_isize_vec(s).with_context(|| "parsing sequence")?;

    if sequence.len() < 2 {
        bail!("sequence must have length 2 or more")
    }

    let mut values = vec![*sequence.last().unwrap()];

    while let Some(diff) = diff_sequence(&sequence) {
        values.push(*diff.last().unwrap());
        sequence = diff;
    }

    Ok(values.iter().sum())
}

pub fn extrapolate_previous(s: &str) -> Result<isize> {
    let mut sequence = parse_isize_vec(s).with_context(|| "parsing sequence")?;

    if sequence.len() < 2 {
        bail!("sequence must have length 2 or more")
    }

    let mut values = vec![*sequence.first().unwrap()];

    while let Some(diff) = diff_sequence(&sequence) {
        values.push(*diff.first().unwrap());
        sequence = diff;
    }

    Ok(values.iter().rev().fold(0, |a, b| b - a))
}

fn diff_sequence(sequence: &[isize]) -> Option<Vec<isize>> {
    if sequence.len() < 2 || sequence.iter().all(|a| a == &0) {
        return None;
    }
    let mut diff = vec![];
    for i in 1..sequence.len() {
        diff.push(sequence[i] - sequence[i - 1]);
    }
    Some(diff)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use test_case::case;

    use super::*;

    #[case("0 3 6 9 12 15", 18 ; "sequence 1")]
    #[case("1 3 6 10 15 21", 28 ; "sequence 2")]
    #[case("10 13 16 21 30 45", 68 ; "sequence 3")]
    fn test_extrapolate_next(input: &str, expected: isize) -> Result<()> {
        assert_eq!(extrapolate_next(input)?, expected);
        Ok(())
    }

    #[case("0 3 6 9 12 15", -3 ; "sequence 1")]
    #[case("1 3 6 10 15 21", 0 ; "sequence 2")]
    #[case("10 13 16 21 30 45", 5 ; "sequence 3")]
    fn test_extrapolate_previous(input: &str, expected: isize) -> Result<()> {
        assert_eq!(extrapolate_previous(input)?, expected);
        Ok(())
    }

    #[case(&[0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3] ; "sequence 1")]
    #[case(&[1, 3, 6, 10, 15, 21], vec![2, 3, 4, 5, 6] ; "sequence 2")]
    #[case(&[10, 13, 16, 21, 30, 45], vec![3, 3, 5, 9, 15] ; "sequence 3")]
    fn test_diff_sequence(input: &[isize], expected: Vec<isize>) {
        assert_eq!(diff_sequence(input), Some(expected));
    }
}
