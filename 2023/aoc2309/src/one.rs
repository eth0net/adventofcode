use std::num::ParseIntError;

use anyhow::{bail, Context, Error, Result};

pub fn extrapolate_sum(s: &str) -> Result<isize> {
    let values = s
        .lines()
        .filter(|line| !line.is_empty())
        .map(extrapolate_sequence)
        .collect::<Result<Vec<isize>, Error>>()
        .with_context(|| "extrapolating sequences")?;
    Ok(values.into_iter().sum())
}

fn parse_isize_vec(s: &str) -> Result<Vec<isize>, Error> {
    s.split_ascii_whitespace()
        .map(str::parse::<isize>)
        .collect::<Result<Vec<isize>, ParseIntError>>()
        .with_context(|| "parsing isize vec from string")
}

fn extrapolate_sequence(s: &str) -> Result<isize> {
    let mut sequence = parse_isize_vec(s).with_context(|| "parsing sequence")?;

    if sequence.len() < 2 {
        bail!("sequence must have length 2 or more")
    }

    let mut values = vec![*sequence.last().unwrap()];

    while let Some(diff) = diff_sequence(&sequence) {
        values.push(*diff.last().unwrap());
        sequence = diff;
    }

    return Ok(values.iter().sum());
}

fn diff_sequence(sequence: &[isize]) -> Option<Vec<isize>> {
    if sequence.len() < 2 {
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

    const EXAMPLE: &str = include_str!("../example");

    #[case(EXAMPLE, 114 ; "example data")]
    fn test_extrapolate_sum(input: &str, expected: isize) -> Result<()> {
        assert_eq!(extrapolate_sum(input)?, expected);
        Ok(())
    }

    #[case("0 3 6 9 12 15", 18 ; "sequence 1")]
    #[case("1 3 6 10 15 21", 28 ; "sequence 2")]
    #[case("10 13 16 21 30 45", 68 ; "sequence 3")]
    fn test_extrapolate_sequence(input: &str, expected: isize) -> Result<()> {
        assert_eq!(extrapolate_sequence(input)?, expected);
        Ok(())
    }

    #[case(&[0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3] ; "sequence 1")]
    #[case(&[1, 3, 6, 10, 15, 21], vec![2, 3, 4, 5, 6] ; "sequence 2")]
    #[case(&[10, 13, 16, 21, 30, 45], vec![3, 3, 5, 9, 15] ; "sequence 3")]
    fn test_diff_sequence(input: &[isize], expected: Vec<isize>) {
        assert_eq!(diff_sequence(input), Some(expected));
    }
}
